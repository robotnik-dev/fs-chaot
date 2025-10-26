use crate::card::{Card, Index, Name};
use anyhow::Result;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use rusqlite::params;

#[cfg(feature = "server")]
thread_local! {
    static DB: std::sync::LazyLock<rusqlite::Connection> = std::sync::LazyLock::new(|| {
        let conn = rusqlite::Connection::open("production.db").expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY,
                name_en TEXT NOT NULL,
                name_de TEXT NOT NULL,
                book INTEGER NOT NULL,
                page INTEGER NOT NULL,
                side TEXT NOT NULL,
                entry INTEGER NOT NULL,
                img_url TEXT NOT NULL,
                owned BOOLEAN NOT NULL CHECK (owned IN (0,1)),
                created_at DATETIME DEFAULT (datetime('now', 'localtime'))
            );",
        )
        .expect("failed to create database table");

        conn
    });
}

#[server(endpoint = "get_card_remote")]
pub async fn get_card_remote(name_or_id: String) -> Result<Card, ServerFnError> {
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

#[server(endpoint = "get_cards_db")]
pub async fn get_cards_db() -> Result<Vec<(usize, Card)>> {
    DB.with(|db| {
        Ok(db
            .prepare(
                "SELECT id, name_en, name_de, book, page, side, entry, img_url, owned FROM cards",
            )?
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    Card {
                        index: row.get(0)?,
                        name_en: row.get(1)?,
                        name_de: row.get(2)?,
                        book: row.get(3)?,
                        page: row.get(4)?,
                        side: row.get(5)?,
                        entry: row.get(6)?,
                        img_url: row.get(7)?,
                        owned: row.get(8)?,
                    },
                ))
            })?
            .collect::<Result<Vec<(usize, Card)>, rusqlite::Error>>()?)
    })
}

#[server(endpoint = "save_card_db")]
pub async fn save_card_db(card: Card) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "INSERT INTO cards (id, name_en, name_de, book, page, side, entry, img_url, owned) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![card.index, card.name_en, card.name_de, card.book, card.page, card.side, card.entry, card.img_url, card.owned],
        )
    })
    .map_err(|err| ServerFnError::ServerError {
        message: err.to_string(),
        code: 500,
        details: Some("could not save card to DB".into()),
    })?;
    Ok(())
}
