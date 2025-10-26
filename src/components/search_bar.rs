use crate::{
    backend::{get_card_remote, save_card_db},
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
                        // search in DB first before calling remote
                        // TODO
                        match get_card_remote(name_or_id).await {
                            Ok(card) => {
                                let mut cards = CARDS.read().clone();
                                cards.push((card.index.0, card.clone()));
                                *CARDS.write() = cards;
                                // save card into DB for quicker search next time
                                if let Err(err) = save_card_db(card).await {
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
