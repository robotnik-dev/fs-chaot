use dioxus::{logger::tracing::info, prelude::*};

use crate::{backend::get_card, card::Card};

mod backend;
mod card;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        SearchBar {}
    }
}

#[component]
fn SearchBar() -> Element {
    let mut current_card: Signal<Option<Card>> = use_signal(|| None);

    rsx! {
        div { id: "input-group",
            input {
                r#type: "text",
                autofocus: true,
                name: "text",
                id: "input",
                placeholder: "Name or ID",
                onkeypress: move |event: Event<KeyboardData>| async move {
                    if event.key() == Key::Enter {
                        // TODO: get input string and parse
                        if let Ok(card) = get_card(1).await {
                            current_card.set(Some(card));
                        }
                        info!("{:?}", current_card);
                    }
                },
            }
        }
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "Chaot" }
        }
    }
}
