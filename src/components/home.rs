use crate::{
    backend::{get_card_by_id_db, get_card_by_id_remote},
    card::{Card, Index},
    components::{CardContainer, CardOwnershipDialog, DialogMode, SearchBar},
    CARDS,
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut dialog_open = use_signal(|| false);
    let mut selected_card = use_signal(Card::default);
    let mut loading_card = use_signal(|| false);
    let mut error_message = use_signal(String::new);

    // Handle card click
    let handle_card_click = move |index: usize| {
        dialog_open.set(true);
        loading_card.set(true);

        spawn(async move {
            // Try to get card from DB first
            match get_card_by_id_db(index).await {
                Ok(card) => {
                    selected_card.set(card);
                    loading_card.set(false);
                }
                Err(_) => {
                    // Card not in DB, fetch from remote
                    match Index::try_new(index) {
                        Ok(idx) => match get_card_by_id_remote(idx.0).await {
                            Ok(card) => {
                                selected_card.set(card);
                                loading_card.set(false);
                            }
                            Err(e) => {
                                error_message.set(format!("Failed to fetch card: {}", e));
                                loading_card.set(false);
                                dialog_open.set(false);
                            }
                        },
                        Err(e) => {
                            error_message.set(format!("Invalid card ID: {}", e));
                            loading_card.set(false);
                            dialog_open.set(false);
                        }
                    }
                }
            }
        });
    };

    // Handle ownership change
    let handle_ownership_change = move |card: Card| {
        // Update global CARDS signal
        let mut cards = CARDS.read().clone();

        if card.owned.0 {
            // Add or update card in global state
            if let Some(pos) = cards.iter().position(|(id, _)| *id == card.index.0) {
                cards[pos] = (card.index.0, card.clone());
            } else {
                cards.push((card.index.0, card.clone()));
            }
        } else {
            // Remove card from global state
            cards.retain(|(id, _)| *id != card.index.0);
        }

        *CARDS.write() = cards;
    };

    let is_owned = move || selected_card.read().clone().owned.0;

    rsx! {
        SearchBar {}
        CardContainer { onclick: handle_card_click }

        CardOwnershipDialog {
            card: selected_card,
            dialog_open,
            mode: if is_owned() { DialogMode::Edit } else { DialogMode::Add },
            on_change: handle_ownership_change,
        }
    }
}
