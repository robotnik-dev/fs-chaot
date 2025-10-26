use crate::{
    backend::{
        get_card_by_id_db, get_card_by_id_remote, get_card_by_name_db, get_card_by_name_remote,
        save_card_db,
    },
    components::{DialogContent, DialogDescription, DialogRoot, DialogTitle},
    CARDS,
};
use dioxus::prelude::*;

#[component]
pub fn SearchBar() -> Element {
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
                        if name_or_id.is_empty() {
                            // search in DB to display the card and dont put duplicates in there
                            // check if this card is already displayed
                            // display card from db and return early
                            return;
                        }

                        let maybe_card_db = {
                            if let Ok(id) = name_or_id.parse::<usize>() {
                                get_card_by_id_db(id).await
                            } else {
                                get_card_by_name_db(name_or_id.clone()).await
                            }
                        };
                        if let Ok(card_db) = maybe_card_db {
                            let mut cards = CARDS.read().clone();
                            if !cards.contains(&(card_db.index.0, card_db.clone())) {
                                cards.push((card_db.index.0, card_db.clone()));
                                *CARDS.write() = cards;
                            }
                            return;
                        }
                        let maybe_card_remote = {
                            if let Ok(id) = name_or_id.parse::<usize>() {
                                get_card_by_id_remote(id).await
                            } else {
                                get_card_by_name_remote(name_or_id.clone()).await
                            }
                        };
                        match maybe_card_remote {
                            Ok(card_remote) => {
                                let mut cards = CARDS.read().clone();
                                cards.push((card_remote.index.0, card_remote.clone()));
                                *CARDS.write() = cards;
                                if let Err(err) = save_card_db(card_remote).await {
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
