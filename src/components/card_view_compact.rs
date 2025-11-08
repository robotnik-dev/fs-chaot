use crate::card::Card;
use dioxus::prelude::*;

#[component]
pub fn CardViewCompact(card: Card, onclick: EventHandler<usize>) -> Element {
    let rarity_class = match card.rarity {
        crate::card::Rarity::Common => "card-compact--rarity-common",
        crate::card::Rarity::Uncommon => "card-compact--rarity-uncommon",
        crate::card::Rarity::Rare => "card-compact--rarity-rare",
        crate::card::Rarity::HoloRare => "card-compact--rarity-holo-rare",
        crate::card::Rarity::UltraRare => "card-compact--rarity-ultra-rare",
        crate::card::Rarity::SecretRare => "card-compact--rarity-secret-rare",
    };

    rsx! {
        div {
            class: "card-compact card-compact--owned {rarity_class}",
            onclick: move |_| onclick.call(card.index.0),
            div { class: "card-compact__sprite",
                img { src: "{card.img_url}", alt: "{card.name_en}" }
            }
            div { class: "card-compact__name--de", "{card.name_de}" }
            div { class: "card-compact__name--en", "{card.name_en}" }
            div { class: "card-compact__meta",
                span { "#{card.index}" }
                span { " • " }
                if card.owned.0 {
                    span { "{card.rarity}" }
                }
            }
        }
    }
}
