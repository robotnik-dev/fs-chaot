use anyhow::Result;
use dioxus::prelude::*;

use crate::card::{Book, Card, Entry, Index, Name, Page, Side};

#[server]
pub async fn get_card(name_or_id: String) -> Result<Card, ServerFnError> {
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

    if let Ok(index) = name_or_id.parse::<usize>() {
        match Index::try_new(index) {
            Ok(index) => Ok(card),
            Err(err) => Err(ServerFnError::ServerError {
                message: err.to_string(),
                code: 500,
                details: None,
            }),
        }
    } else {
        // TODO: is string
        Err(ServerFnError::ServerError {
            message: "unimplemented error".to_string(),
            code: 500,
            details: None,
        })
    }
}
