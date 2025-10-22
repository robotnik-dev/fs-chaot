use std::collections::HashMap;

use dioxus::prelude::*;
use itertools::Itertools;

use crate::{
    backend::{get_card_remote, save_searched_card_db},
    card::Card,
    components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle},
};

mod backend;
mod card;
mod components;
mod csv_record;
mod pokeapi;

pub const BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
pub const LANGUAGE_URL: &str = "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv/pokemon_species_names.csv";
pub const SPRITE_URL: &str =
    "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/";
pub const CARDS_PER_BOOK: usize = 576;
pub const CARDS_PER_PAGE: usize = 24;
static STYLE: Asset = asset!("/assets/style.css");
static THEME: Asset = asset!("/assets/dx-components-theme.css");

fn main() {
    dioxus::launch(App);
}

static CARDS: GlobalSignal<HashMap<usize, Card>> = Signal::global(HashMap::new);

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: STYLE }
        document::Stylesheet { href: THEME }
        Title {}
        SearchBar {}
        CardContainer {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { class: "title",
            h1 { "C.h.a.o.t" }
        }
    }
}

#[component]
fn SearchBar() -> Element {
    let mut search = use_signal(|| "".to_string());
    let mut open = use_signal(|| false);
    let mut error_message = use_signal(|| "".to_string());

    rsx! {
        div { class: "input-group",
            input {
                r#type: "text",
                autofocus: true,
                name: "text",
                class: "input",
                placeholder: "Name or ID",
                oninput: move |event| search.set(event.value().clone()),
                onkeypress: move |event: Event<KeyboardData>| async move {
                    if event.key() == Key::Enter {
                        let name_or_id = search.peek().to_string();
                        // search in DB first before calling remote
                        // TODO
                        match get_card_remote(name_or_id).await {
                            Ok(card) => {
                                let mut cards = CARDS.read().clone();
                                cards.insert(card.index.0, card.clone());
                                *CARDS.write() = cards;
                                // save card into DB for quicker search next time
                                if let Err(err) = save_searched_card_db(card).await {
                                    error_message.set(err.to_string());
                                    open.set(true);
                                }
                            }
                            Err(err) => {
                                error_message.set(err.to_string());
                                open.set(true);
                            }
                        }
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

#[component]
fn CardContainer() -> Element {
    let cards = CARDS.read().clone();
    rsx! {
        div { class: "card-container",
            {
                cards
                    .iter()
                    .sorted_by_key(|(index, _)| **index)
                    .map(|(_, card)| rsx! {
                        CardView { card: card.clone() }
                    })
            }
        }
    }
}

#[component]
fn CardView(card: Card) -> Element {
    rsx! {
        div { class: "card",
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
            }
        }
    }
}
