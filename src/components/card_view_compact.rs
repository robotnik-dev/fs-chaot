use crate::{
    card::{Card, Rarity},
    utils::get_highest_rarity,
};
use dioxus::prelude::*;

#[component]
pub fn CardViewCompact(card: Card, onclick: EventHandler<usize>) -> Element {
    let mut rarity_class = use_signal(String::new);

    use_effect(move || {
        spawn(async move {
            let class = match get_highest_rarity(card.index.0).await {
                Rarity::Common => "card-compact--rarity-common",
                Rarity::Uncommon => "card-compact--rarity-uncommon",
                Rarity::Rare => "card-compact--rarity-rare",
                Rarity::HoloRare => "card-compact--rarity-holo-rare",
                Rarity::ReverseHoloRare => "card-compact--rarity-reverse-holo-rare",
                Rarity::DoubleRare => "card-compact--rarity-double-rare",
                Rarity::UltraRare => "card-compact--rarity-ultra-rare",
                Rarity::SecretRare => "card-compact--rarity-secret-rare",
                Rarity::Promo => "card-compact--rarity-promo",
            };
            rarity_class.set(class.to_string());
        });
    });

    rsx! {
        div {
            class: "card-compact card-compact--owned {rarity_class}",
            onclick: move |_| onclick.call(card.index.0),
            div { class: "card-compact__meta",
                span { "#{card.index}" }
            }
            div { class: "card-compact__sprite",
                img { src: "{card.img_url}", alt: "{card.name_en}" }
            }
            div { class: "card-compact__name--de", "{card.name_de}" }
            div { class: "card-compact__name--en", "{card.name_en}" }
        }
    }
}
