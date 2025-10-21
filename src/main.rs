use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    backend::get_card,
    card::Card,
    components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle},
};

mod backend;
mod card;
mod components;

pub const BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
pub const LANGUAGE_URL: &str = "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv/pokemon_species_names.csv";
pub const CARDS_PER_BOOK: usize = 576;
pub const CARDS_PER_PAGE: usize = 24;
static MAIN_CSS: Asset = asset!("/assets/main.css");
static COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: COMPONENTS_CSS }
        Title {}
        SearchBar {}
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

#[component]
fn SearchBar() -> Element {
    let mut current_card: Signal<Option<Card>> = use_signal(|| None);
    let mut search = use_signal(|| "".to_string());
    let mut open = use_signal(|| false);
    let mut error_message = use_signal(|| "".to_string());

    rsx! {
        div { id: "input-group",
            input {
                r#type: "text",
                autofocus: true,
                name: "text",
                id: "input",
                placeholder: "Name or ID",
                oninput: move |event| search.set(event.value().clone()),
                onkeypress: move |event: Event<KeyboardData>| async move {
                    if event.key() == Key::Enter {
                        let name_or_id = search.peek().to_string();
                        match get_card(name_or_id).await {
                            Ok(card) => current_card.set(Some(card)),
                            Err(err) => {
                                error_message.set(err.to_string());
                                open.set(true);
                            }
                        }
                        info!("{:?}", current_card);
                    }
                },
            }
            DialogRoot { open: open(), on_open_change: move |v| open.set(v),
                DialogContent {
                    button {
                        class: "dialog-close",
                        r#type: "button",
                        aria_label: "Close",
                        tabindex: if open() { "0" } else { "-1" },
                        onclick: move |_| open.set(false),
                        "×"
                    }
                    DialogTitle { "Error" }
                    DialogDescription { {error_message.read().to_string()} }
                }
            }
        }
    }
}
