use crate::{backend::get_cards_with_timestamp_db, components::CardView};
use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn History() -> Element {
    let cards = use_loader(get_cards_with_timestamp_db)?;

    rsx! {
        div { class: "card-container",
            {
                cards
                    .cloned()
                    .iter()
                    .sorted_by_key(|(_, ts)| ts)
                    .rev()
                    .take(12) // TODO: only the latest 12 entries for now to avoid having all of them, make interactive
                    .map(|(card, _)| rsx! {
                        CardView { card: card.clone() }
                    })
            }
        }
    }
}
