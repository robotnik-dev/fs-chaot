use crate::{backend::get_cards_db, components::CardView};
use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn History() -> Element {
    let cards = use_loader(get_cards_db)?;
    rsx! {
        div { class: "card-container",
            {
                cards
                    .cloned()
                    .iter()
                    .sorted_by_key(|(index, _)| index)
                    .take(12) // TODO: only the latest 12 entries for now to avoid having all of them
                    .map(|(_, card)| rsx! {
                        CardView { card: card.clone() }
                    })
            }
        }
    }
}
