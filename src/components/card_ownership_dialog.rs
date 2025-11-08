use crate::{
    backend::{
        delete_all_card_expansions_db, get_all_expansions_db, get_card_expansions_db, save_card_db,
        save_card_expansion_db, update_card_db,
    },
    card::{Bool, Card},
    components::{DialogContent, DialogRoot, DialogTitle},
    expansion::{CardExpansion, Expansion},
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum DialogMode {
    AddAndRemove, // All views - can add or remove owned cards
}

#[derive(Clone, PartialEq)]
struct ExpansionEntry {
    id: Option<usize>, // None for new entries
    expansion_id: usize,
    card_number: String,
}

#[component]
pub fn CardOwnershipDialog(
    card: Signal<Option<Card>>,
    dialog_open: Signal<bool>,
    mode: DialogMode,
    on_change: EventHandler<Card>,
) -> Element {
    let mut all_expansions = use_signal(Vec::<Expansion>::new);
    let mut card_expansions = use_signal(Vec::<ExpansionEntry>::new);
    let mut loading_expansions = use_signal(|| false);
    let mut error_message = use_signal(String::new);
    let mut is_submitting = use_signal(|| false);

    // New expansion form state
    let mut new_expansion_id = use_signal(|| None::<usize>);
    let mut new_card_number = use_signal(String::new);

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
        if dialog_open() {
            if let Some(c) = card.read().clone() {
                if c.owned.0 {
                    spawn(async move {
                        match get_card_expansions_db(c.index.0).await {
                            Ok(expansions) => {
                                let entries: Vec<ExpansionEntry> = expansions
                                    .into_iter()
                                    .map(|ce| ExpansionEntry {
                                        id: ce.id,
                                        expansion_id: ce.expansion_id,
                                        card_number: ce.card_number,
                                    })
                                    .collect();
                                card_expansions.set(entries);
                            }
                            Err(e) => {
                                error_message.set(format!("Failed to load card expansions: {}", e));
                            }
                        }
                    });
                }
            }
        } else {
            // Reset form when dialog closes
            card_expansions.set(Vec::new());
            new_expansion_id.set(None);
            new_card_number.set(String::new());
            error_message.set(String::new());
        }
    });

    // Add expansion to list
    let add_expansion = move |_| {
        if let Some(exp_id) = new_expansion_id() {
            if !new_card_number().trim().is_empty() {
                let mut expansions = card_expansions.read().clone();

                // Check for duplicates
                if expansions.iter().any(|e| e.expansion_id == exp_id) {
                    error_message.set("This expansion is already added".to_string());
                    return;
                }

                expansions.push(ExpansionEntry {
                    id: None,
                    expansion_id: exp_id,
                    card_number: new_card_number(),
                });
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

        if let Some(mut c) = card.read().clone() {
            is_submitting.set(true);
            c.owned = Bool(true);

            let card_clone = c.clone();
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
                if let Err(e) = delete_all_card_expansions_db(card_clone.index.0).await {
                    error_message.set(format!("Failed to delete expansions: {}", e));
                    is_submitting.set(false);
                    return;
                }

                // Insert all expansions from the current list
                for entry in expansions_clone.iter() {
                    let card_expansion = CardExpansion {
                        id: None, // Always insert as new (id will be auto-generated)
                        card_id: card_clone.index.0,
                        expansion_id: entry.expansion_id,
                        card_number: entry.card_number.clone(),
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
                on_change.call(card_clone);
            });
        }
    };

    // Handle remove from collection
    let handle_remove_from_collection = move |_| {
        if let Some(mut c) = card.read().clone() {
            is_submitting.set(true);
            c.owned = Bool(false);

            let card_clone = c.clone();

            spawn(async move {
                // Delete all expansion associations
                let delete_result = delete_all_card_expansions_db(card_clone.index.0).await;

                if let Err(e) = delete_result {
                    error_message.set(format!("Failed to delete expansions: {}", e));
                    is_submitting.set(false);
                    return;
                }

                // Update card to owned=false
                let update_result = update_card_db(card_clone.clone()).await;

                if let Err(e) = update_result {
                    error_message.set(format!("Failed to update card: {}", e));
                    is_submitting.set(false);
                    return;
                }

                // Success
                is_submitting.set(false);
                dialog_open.set(false);
                on_change.call(card_clone);
            });
        }
    };

    rsx! {
        DialogRoot { open: dialog_open(), on_open_change: move |v| dialog_open.set(v),
            DialogContent {
                button {
                    class: "dialog-close",
                    r#type: "button",
                    aria_label: "Close",
                    onclick: move |_| dialog_open.set(false),
                    "×"
                }

                DialogTitle { "Card Details" }

                if let Some(c) = card.read().clone() {
                    div { class: "card-dialog-content",
                        // Card Image
                        div { class: "card-dialog-image",
                            img { src: "{c.img_url}", alt: "{c.name_en}" }
                        }

                        // Card Names
                        div { class: "card-dialog-name-de", "{c.name_de}" }
                        div { class: "card-dialog-name-en", "{c.name_en}" }

                        // Card Info
                        div { class: "card-dialog-info",
                            div { "ID: #{c.index}" }
                            div { "Rarity: {c.rarity}" }
                        }

                        // Expansion Management (only for AddAndRemove mode or if owned)
                        if matches!(mode, DialogMode::AddAndRemove) || c.owned.0 {
                            div { class: "expansion-manager",
                                h3 { class: "expansion-manager-title", "Expansions" }

                                // Current expansions list
                                if !card_expansions().is_empty() {
                                    div { class: "expansion-list",
                                        for (index , entry) in card_expansions().iter().enumerate() {
                                            div {
                                                class: "expansion-item",
                                                key: "{index}",
                                                div { class: "expansion-item-info",
                                                    if let Some(exp) = all_expansions().iter().find(|e| e.id == entry.expansion_id) {
                                                        span { class: "expansion-name",
                                                            "{exp.name}"
                                                        }
                                                        span { class: "expansion-card-num",
                                                            " - Card #{entry.card_number}"
                                                        }
                                                    }
                                                }
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

                                // Add new expansion form (only if not owned or in AddAndRemove mode)
                                if matches!(mode, DialogMode::AddAndRemove) || c.owned.0 {
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

                                        button {
                                            class: "btn-add-expansion",
                                            r#type: "button",
                                            onclick: add_expansion,
                                            "+ Add Expansion"
                                        }
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
                            // Add to Collection button (only in AddAndRemove mode and not owned)
                            if matches!(mode, DialogMode::AddAndRemove) && !c.owned.0 {
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

                            // Save Changes button (if owned and has expansions)
                            if c.owned.0 && matches!(mode, DialogMode::AddAndRemove) {
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
                            }

                            // Remove from Collection button
                            if c.owned.0 {
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
