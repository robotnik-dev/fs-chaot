use crate::{components::CardView, CARDS};
use dioxus::prelude::*;
use itertools::Itertools;

#[component]
pub fn CardContainer(onclick: Option<EventHandler<usize>>) -> Element {
    let cards = CARDS.read().clone();
    rsx! {
        div { class: "card-container",
            {cards.iter().sorted_by_key(|(index, _)| index).map(|(_, card)| rsx! {
                CardView { card: card.clone(), onclick }
            })}
        }
    }
}
