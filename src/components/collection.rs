use crate::{
    backend::{
        get_all_owned_cards_db, get_card_by_id_db, get_card_by_id_remote, get_card_by_name_db,
        get_card_by_name_remote, save_card_db,
    },
    card::Card,
    components::{
        BookNavigation, CardOwnershipDialog, CardViewCompact, DialogContent, DialogDescription,
        DialogMode, DialogRoot, DialogTitle, PlaceholderCard,
    },
    CARDS_PER_DOUBLE_PAGE, MAX_POKEMON, TOTAL_PAGES,
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Collection() -> Element {
    let mut current_page = use_signal(|| 1usize);
    let mut owned_cards = use_signal(HashMap::<usize, Card>::new);
    let mut dialog_open = use_signal(|| false);
    let mut selected_index = use_signal(|| None::<usize>);
    let mut error_message = use_signal(String::new);
    let mut loading_card = use_signal(|| false);
    let mut temp_card = use_signal(Card::default);

    // Check if mobile viewport
    let is_mobile = use_signal(|| {
        #[cfg(all(feature = "web", target_arch = "wasm32"))]
        {
            use wasm_bindgen::JsCast;
            web_sys::window()
                .and_then(|w| w.inner_width().ok())
                .and_then(|w| w.as_f64())
                .map(|w| w < 768.0)
                .unwrap_or(false)
        }
        #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
        {
            false
        }
    });

    // Load owned cards on mount
    use_effect(move || {
        spawn(async move {
            match get_all_owned_cards_db().await {
                Ok(cards) => {
                    owned_cards.set(cards);
                }
                Err(e) => {
                    error_message.set(format!("Failed to load collection: {}", e));
                }
            }
        });
    });

    // Handle card click
    let handle_card_click = move |index: usize| {
        selected_index.set(Some(index));
        dialog_open.set(true);

        // Check if card is owned
        if let Some(card) = owned_cards.read().get(&index) {
            temp_card.set(card.clone());
        } else {
            loading_card.set(true);
            spawn(async move {
                // Fetch from db
                match get_card_by_id_db(index).await {
                    Ok(card) => {
                        temp_card.set(card.clone());
                        loading_card.set(false);
                    }

                    Err(_) => {
                        // Fetch from remote
                        match get_card_by_id_remote(index).await {
                            Ok(card) => {
                                temp_card.set(card.clone());
                                loading_card.set(false);
                                if let Err(e) = save_card_db(card.clone()).await {
                                    error_message.set(format!("Failed to save card: {}", e));
                                    loading_card.set(false);
                                    dialog_open.set(false);
                                }
                            }
                            Err(e) => {
                                error_message.set(format!("Failed to fetch card: {}", e));
                                loading_card.set(false);
                                dialog_open.set(false);
                            }
                        }
                    }
                }
            });
        }
    };

    // Handle ownership change (from dialog)
    let handle_ownership_change = move |card: Card| {
        let index = card.index.0;

        if card.owned.0 {
            // Add to collection
            owned_cards.write().insert(index, card.clone());
        } else {
            // Remove from collection
            owned_cards.write().remove(&index);
        }
    };

    // Handle search
    let handle_search = move |input: String| {
        if input.is_empty() {
            return;
        }

        // Try parsing as ID first
        if let Ok(index) = input.parse::<usize>() {
            selected_index.set(Some(index));
            loading_card.set(true);

            spawn(async move {
                // Fetch from db
                match get_card_by_id_db(index).await {
                    Ok(card) => {
                        current_page.set(card.page.0);
                        temp_card.set(card.clone());
                        loading_card.set(false);
                        dialog_open.set(true);
                    }

                    Err(_) => {
                        // Fetch from remote
                        match get_card_by_id_remote(index).await {
                            Ok(card) => {
                                current_page.set(card.page.0);
                                temp_card.set(card.clone());
                                loading_card.set(false);
                                dialog_open.set(true);
                                if let Err(e) = save_card_db(card.clone()).await {
                                    error_message.set(format!("Failed to save card: {}", e));
                                    loading_card.set(false);
                                    dialog_open.set(false);
                                }
                            }
                            Err(e) => {
                                error_message.set(format!("Failed to fetch card: {}", e));
                                loading_card.set(false);
                                dialog_open.set(false);
                            }
                        }
                    }
                }
            });
        } else {
            loading_card.set(true);

            spawn(async move {
                // Fetch from db
                match get_card_by_name_db(input.clone()).await {
                    Ok(card) => {
                        current_page.set(card.page.0);
                        temp_card.set(card.clone());
                        loading_card.set(false);
                        selected_index.set(Some(card.index.0));
                        dialog_open.set(true);
                    }

                    Err(_) => {
                        // Fetch from remote
                        match get_card_by_name_remote(input).await {
                            Ok(card) => {
                                current_page.set(card.page.0);
                                temp_card.set(card.clone());
                                loading_card.set(false);
                                selected_index.set(Some(card.index.0));
                                dialog_open.set(true);
                                if let Err(e) = save_card_db(card.clone()).await {
                                    error_message.set(format!("Failed to save card: {}", e));
                                    loading_card.set(false);
                                    dialog_open.set(false);
                                }
                            }
                            Err(e) => {
                                error_message.set(format!("Failed to fetch card: {}", e));
                                loading_card.set(false);
                                dialog_open.set(false);
                            }
                        }
                    }
                }
            });
        }
    };

    // Calculate cards for current page
    let cards_for_page = move || {
        let page = current_page();
        let start = (page - 1) * CARDS_PER_DOUBLE_PAGE + 1;
        let end = (start + CARDS_PER_DOUBLE_PAGE - 1).min(MAX_POKEMON);
        (start..=end).collect::<Vec<_>>()
    };

    // Render page side (12 cards)
    let render_page_side = move |indices: Vec<usize>| {
        rsx! {
            div { class: "book-page",
                {
                    indices
                        .into_iter()
                        .map(|idx| {
                            let owned = owned_cards.read();
                            if let Some(card) = owned.get(&idx) {
                                rsx! {
                                    CardViewCompact { key: "{idx}", card: card.clone(), onclick: handle_card_click }
                                }
                            } else {
                                rsx! {
                                    PlaceholderCard { key: "{idx}", index: idx, onclick: handle_card_click }
                                }
                            }
                        })
                }
            }
        }
    };

    let is_owned = move || temp_card.read().clone().owned.0;

    rsx! {
        div { class: "collection-container",
            BookNavigation {
                current_page,
                total_pages: TOTAL_PAGES,
                on_search: handle_search,
                loading_card,
            }

            div { class: if is_mobile() { "book-view-mobile" } else { "book-view-desktop" },
                div { class: "book-spread",
                    // Left page (first 12 cards)
                    {
                        let cards = cards_for_page();
                        let left_cards = cards[..12.min(cards.len())].to_vec();
                        render_page_side(left_cards)
                    }
                    // Right page (last 12 cards)
                    {
                        let cards = cards_for_page();
                        if cards.len() > 12 {
                            let right_cards = cards[12..].to_vec();
                            render_page_side(right_cards)
                        } else {
                            rsx! {
                                div { class: "book-page book-page--empty" }
                            }
                        }
                    }
                }
            }

            // Card ownership dialog
            CardOwnershipDialog {
                card: temp_card,
                dialog_open,
                mode: if is_owned() { DialogMode::Edit } else { DialogMode::Add },
                on_change: handle_ownership_change,
            }

            // Error dialog
            if !error_message.read().is_empty() {
                DialogRoot {
                    open: true,
                    on_open_change: move |v: bool| {
                        if !v {
                            error_message.set(String::new());
                        }
                    },
                    DialogContent {
                        button {
                            class: "dialog-close",
                            r#type: "button",
                            aria_label: "Close",
                            onclick: move |_| error_message.set(String::new()),
                            "×"
                        }
                        DialogTitle { "Notice" }
                        DialogDescription { {error_message.read().clone()} }
                    }
                }
            }
        }
    }
}
