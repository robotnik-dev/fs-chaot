use dioxus::prelude::*;

use crate::card::{Book, Card, Entry, Index, Name, Page, Side};

#[server]
pub async fn get_card(id: usize) -> Result<Card, ServerFnError> {
    // TODO: real impl
    let card = Card {
        index: Index(1),
        name_en: Name("Some name".into()),
        name_de: Name("Ein Name".into()),
        book: Book(1),
        page: Page(1),
        side: Side::A,
        entry: Entry(1),
    };

    Ok(card)
}
