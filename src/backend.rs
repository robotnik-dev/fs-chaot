use anyhow::Result;
use dioxus::prelude::*;

use crate::card::{Card, Index, Name};

#[server]
pub async fn get_card(name_or_id: String) -> Result<Card, ServerFnError> {
    if let Ok(index) = name_or_id.parse::<usize>() {
        match Index::try_new(index) {
            Ok(index) => Ok(Card::try_from_index(index).await?),
            Err(err) => Err(ServerFnError::ServerError {
                message: err.to_string(),
                code: 500,
                details: None,
            }),
        }
    } else {
        Ok(Card::try_from_name(Name::new(name_or_id.as_str())).await?)
    }
}
