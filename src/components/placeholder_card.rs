use dioxus::prelude::*;

#[component]
pub fn PlaceholderCard(index: usize, onclick: EventHandler<usize>) -> Element {
    rsx! {
        div {
            class: "card-compact card-compact--placeholder",
            onclick: move |_| onclick.call(index),
            div { class: "card-compact__placeholder-icon", "?" }
            div { class: "card-compact__id", "#{index}" }
            div { class: "card-compact__placeholder-text", "Not Owned" }
        }
    }
}
