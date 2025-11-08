use crate::card::Card;
use dioxus::prelude::*;

#[component]
pub fn CardView(card: Card, onclick: Option<EventHandler<usize>>) -> Element {
    let clickable = onclick.is_some();
    let class_name = if clickable {
        "card card-clickable"
    } else {
        "card"
    };

    rsx! {
        div {
            class: "{class_name}",
            onclick: move |_| {
                if let Some(handler) = onclick {
                    handler.call(card.index.0);
                }
            },
            div { class: "card-title", "{card.name_de}" }
            div { class: "card-image",
                img { src: "{card.img_url}" }
            }
            div { class: "card-info",
                div { class: "card-row",
                    span { class: "card-label", "Index:" }
                    span { class: "card-value", "{card.index}" }
                }
                div { class: "card-row",
                    span { class: "card-label", "Name (EN):" }
                    span { class: "card-value", "{card.name_en}" }
                }
                div { class: "card-row",
                    span { class: "card-label", "Book:" }
                    span { class: "card-value", "{card.book}" }
                }
                div { class: "card-row",
                    span { class: "card-label", "Page:" }
                    span { class: "card-value", "{card.page}" }
                }
                div { class: "card-row",
                    span { class: "card-label", "Side:" }
                    span { class: "card-value", "{card.side}" }
                }
                div { class: "card-row",
                    span { class: "card-label", "Entry:" }
                    span { class: "card-value", "{card.entry}" }
                }
                div { class: "card-row",
                    span { class: "card-label", "Owned:" }
                    span {
                        class: "card-value",
                        style: if card.owned.0 { "color:green" } else { "color:red" },
                        "{card.owned}"
                    }
                }
            }
        }
    }
}
