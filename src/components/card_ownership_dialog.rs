use crate::{
    backend::{
        delete_all_card_expansions_db, get_all_expansions_db, get_card_expansions_db, save_card_db,
        save_card_expansion_db, update_card_db,
    },
    card::{Bool, Card, Rarity},
    components::{DialogContent, DialogRoot, DialogTitle},
    expansion::{CardExpansion, Expansion},
    utils::get_highest_rarity,
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

#[derive(Clone, PartialEq)]
pub enum DialogMode {
    Read,
    Add,
    Edit,
}

#[derive(Default, Clone, PartialEq)]
pub struct ExpansionEntry {
    pub id: Option<usize>, // None for new entries
    pub expansion_id: usize,
    pub card_number: String,
    pub rarity: Rarity,
}

#[component]
pub fn CardOwnershipDialog(
    card: Signal<Card>,
    dialog_open: Signal<bool>,
    mode: DialogMode,
    on_change: EventHandler<Card>,
) -> Element {
    let mut all_expansions = use_signal(Vec::<Expansion>::new);
    let mut all_rarities = use_signal(Vec::<Rarity>::new);
    let mut loading_expansions = use_signal(|| false);
    let mut error_message = use_signal(String::new);
    let mut is_submitting = use_signal(|| false);
    let mut card_expansions = use_signal(Vec::<ExpansionEntry>::new);
    let mut highest_rarity = use_signal(Rarity::default);

    // New expansion form state
    let mut new_expansion_id = use_signal(|| None::<usize>);
    let mut new_card_number = use_signal(String::new);
    let mut new_rarity = use_signal(|| Rarity::Common);

    // Load all rarities on mount
    use_effect(move || {
        spawn(async move {
            let mut rarities = vec![];
            for rarity in Rarity::iter() {
                rarities.push(rarity);
            }
            all_rarities.set(rarities);
        });
    });

    // Load all expansions on mount
    use_effect(move || {
        spawn(async move {
            loading_expansions.set(true);
            match get_all_expansions_db().await {
                Ok(expansions) => {
                    all_expansions.set(expansions);
                }
                Err(e) => {
                    error_message.set(format!("Failed to load expansions: {}", e));
                }
            }
            loading_expansions.set(false);
        });
    });

    // Load card expansions when dialog opens
    use_effect(move || {
        if card.cloned().owned.0 {
            spawn(async move {
                let index = card.peek().index.0;
                match get_card_expansions_db(index).await {
                    Ok(expansions) => {
                        let entries: Vec<ExpansionEntry> = expansions
                            .into_iter()
                            .map(|ce| ExpansionEntry {
                                id: ce.id,
                                expansion_id: ce.expansion_id,
                                card_number: ce.card_number,
                                rarity: ce.rarity,
                            })
                            .collect();
                        card_expansions.set(entries);
                        // set highest rarity
                        let rarity = get_highest_rarity(index).await;
                        highest_rarity.set(rarity);
                    }
                    Err(e) => {
                        error_message.set(format!("Failed to load card expansions: {}", e));
                    }
                }
            });
        }
    });

    // Add expansion to list
    let add_expansion = move |_| {
        if let Some(exp_id) = new_expansion_id() {
            if !new_card_number().trim().is_empty() {
                let mut expansions = card_expansions.read().clone();

                // Check for duplicates
                if let Some((index, _)) = expansions
                    .iter()
                    .enumerate()
                    .find(|(_, ex)| ex.expansion_id == exp_id)
                {
                    let ex = expansions.swap_remove(index);
                    let entry = ExpansionEntry {
                        card_number: new_card_number(),
                        rarity: new_rarity(),
                        ..ex
                    };
                    expansions.push(entry);
                } else {
                    // push new expansion
                    expansions.push(ExpansionEntry {
                        id: None,
                        expansion_id: exp_id,
                        card_number: new_card_number(),
                        rarity: new_rarity(),
                    });
                }

                card_expansions.set(expansions);

                // Reset form
                new_expansion_id.set(None);
                new_card_number.set(String::new());
                error_message.set(String::new());
            } else {
                error_message.set("Card number is required".to_string());
            }
        } else {
            error_message.set("Please select an expansion".to_string());
        }
    };

    // Remove expansion from list
    let mut remove_expansion = move |index: usize| {
        let mut expansions = card_expansions.read().clone();

        // Prevent removing the last expansion
        if expansions.len() <= 1 {
            error_message.set("At least one expansion is required".to_string());
            return;
        }

        if index < expansions.len() {
            expansions.remove(index);
            card_expansions.set(expansions);
            error_message.set(String::new());
        }
    };

    // Handle add to collection
    let handle_add_to_collection = move |_| {
        if card_expansions().is_empty() {
            error_message.set("At least one expansion is required".to_string());
            return;
        }

        is_submitting.set(true);
        card.write().owned = Bool(true);
        let card_clone = card.read().clone();

        let expansions_clone = card_expansions();

        spawn(async move {
            // Save or update card
            let save_result = save_card_db(card_clone.clone()).await;

            if let Err(e) = save_result {
                error_message.set(format!("Failed to save card: {}", e));
                is_submitting.set(false);
                return;
            }

            // Delete all existing expansions for this card
            if let Err(e) = delete_all_card_expansions_db(card.cloned().index.0).await {
                error_message.set(format!("Failed to delete expansions: {}", e));
                is_submitting.set(false);
                return;
            }

            // Insert all expansions from the current list
            for entry in expansions_clone.iter() {
                let card_expansion = CardExpansion {
                    id: None, // Always insert as new (id will be auto-generated)
                    card_id: card.cloned().index.0,
                    expansion_id: entry.expansion_id,
                    card_number: entry.card_number.clone(),
                    rarity: entry.rarity.clone(),
                };

                if let Err(e) = save_card_expansion_db(card_expansion).await {
                    error_message.set(format!("Failed to save expansion: {}", e));
                    is_submitting.set(false);
                    return;
                }
            }

            // Success
            is_submitting.set(false);
            dialog_open.set(false);
            on_change.call(card.cloned());
        });
    };

    // Handle remove from collection
    let handle_remove_from_collection = move |_| {
        is_submitting.set(true);
        card.write().owned = Bool(false);

        spawn(async move {
            // Delete all expansion associations
            let delete_result = delete_all_card_expansions_db(card.cloned().index.0).await;

            if let Err(e) = delete_result {
                error_message.set(format!("Failed to delete expansions: {}", e));
                is_submitting.set(false);
                return;
            }

            // Update card to owned=false
            let update_result = update_card_db(card.cloned()).await;

            if let Err(e) = update_result {
                error_message.set(format!("Failed to update card: {}", e));
                is_submitting.set(false);
                return;
            }

            // Success
            is_submitting.set(false);
            dialog_open.set(false);
            on_change.call(card.cloned());
        });
    };

    rsx! {
        DialogRoot {
            open: dialog_open(),
            on_open_change: move |v| {
                dialog_open.set(v);
                card_expansions.set(Vec::new());
                new_expansion_id.set(None);
                new_card_number.set(String::new());
                error_message.set(String::new());
            },
            DialogContent {
                button {
                    class: "dialog-close",
                    r#type: "button",
                    aria_label: "Close",
                    onclick: move |_| dialog_open.set(false),
                    "×"
                }

                DialogTitle { "Card Details" }

                div { class: "card-dialog-content",
                    // Card Image
                    div { class: "card-dialog-image",
                        img {
                            src: "{card.cloned().img_url}",
                            alt: "{card.cloned().name_en}",
                        }
                    }

                    // Card Names
                    div { class: "card-dialog-name-de", "{card.cloned().name_de}" }
                    div { class: "card-dialog-name-en", "{card.cloned().name_en}" }

                    // Card Info
                    div { class: "card-dialog-info",
                        div { "ID: #{card.cloned().index}" }
                        if !matches!(mode, DialogMode::Read) {
                            div { "Rarity: {highest_rarity.cloned()}" }
                        }
                    }
                    if !matches!(mode, DialogMode::Read) {
                        div { class: "expansion-manager",
                            h3 { class: "expansion-manager-title", "Expansions" }

                            // Current expansions list
                            div { class: "expansion-list",
                                for (index , entry) in card_expansions().iter().enumerate() {
                                    div {
                                        class: "expansion-item",
                                        key: "{index}",
                                        div { class: "expansion-item-info",
                                            if let Some(exp) = all_expansions().iter().find(|e| e.id == entry.expansion_id) {
                                                span { class: "expansion-name", "{exp.name}" }
                                                span { class: "expansion-card-num",
                                                    " - Card #{entry.card_number}"
                                                }
                                                span { class: "expansion-card-num", " - {entry.rarity}" }
                                            }
                                        }
                                        if matches!(mode, DialogMode::Edit) {
                                            button {
                                                class: "btn-delete-expansion",
                                                r#type: "button",
                                                onclick: move |_| remove_expansion(index),
                                                "Remove"
                                            }
                                        }
                                    }
                                }
                            }

                            // Add new expansion form
                            if matches!(mode, DialogMode::Add) || matches!(mode, DialogMode::Edit) {
                                div { class: "expansion-selector",
                                    h4 { "Add Expansion" }

                                    select {
                                        class: "expansion-dropdown",
                                        value: new_expansion_id().map(|id| id.to_string()).unwrap_or_default(),
                                        onchange: move |evt| {
                                            if let Ok(id) = evt.value().parse::<usize>() {
                                                new_expansion_id.set(Some(id));
                                            } else {
                                                new_expansion_id.set(None);
                                            }
                                        },
                                        option { value: "", "Select expansion..." }
                                        for expansion in all_expansions().iter() {
                                            option { value: "{expansion.id}", "{expansion.name}" }
                                        }
                                    }

                                    input {
                                        class: "card-number-input",
                                        r#type: "text",
                                        placeholder: "Card number (e.g., 25/102)",
                                        value: "{new_card_number()}",
                                        oninput: move |evt| new_card_number.set(evt.value()),
                                    }

                                    select {
                                        class: "rarity-dropdown",
                                        value: new_rarity().to_string(),
                                        onchange: move |evt| {
                                            if let Ok(rarity) = evt.value().parse::<Rarity>() {
                                                new_rarity.set(rarity);
                                            }
                                        },
                                        option { value: "", "Select rarity..." }
                                        for rarity in all_rarities().iter() {
                                            option { value: "{rarity}", "{rarity}" }
                                        }
                                    }

                                    button {
                                        class: "btn-add-expansion",
                                        r#type: "button",
                                        onclick: add_expansion,
                                        "+ Add Expansion"
                                    }
                                }
                            }
                        }
                        // Error message
                        if !error_message().is_empty() {
                            div { class: "expansion-error", "{error_message()}" }
                        }

                        // Action buttons
                        div { class: "card-dialog-actions",
                            // Add to Collection button
                            if matches!(mode, DialogMode::Add) {
                                button {
                                    class: "btn-add",
                                    disabled: is_submitting(),
                                    onclick: handle_add_to_collection,
                                    if is_submitting() {
                                        "Adding..."
                                    } else {
                                        "Add to Collection"
                                    }
                                }
                            }

                            // Save Changes and Remove from Collection button
                            if matches!(mode, DialogMode::Edit) {
                                button {
                                    class: "btn-add",
                                    disabled: is_submitting() || card_expansions().is_empty(),
                                    onclick: handle_add_to_collection,
                                    if is_submitting() {
                                        "Saving..."
                                    } else {
                                        "Save Changes"
                                    }
                                }
                                button {
                                    class: "btn-remove",
                                    disabled: is_submitting(),
                                    onclick: handle_remove_from_collection,
                                    if is_submitting() {
                                        "Removing..."
                                    } else {
                                        "Remove from Collection"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
