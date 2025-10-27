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

#[server(endpoint = "validate_password")]
pub async fn validate_password(password: String) -> Result<bool, ServerFnError> {
    let correct_password = std::env::var("APP_PASSWORD").unwrap();

    Ok(password == correct_password)
}

#[server(endpoint = "get_card_by_id_remote")]
pub async fn get_card_by_id_remote(id: usize) -> Result<Card, ServerFnError> {
    info!("get card from remote with id: {id}");
    match Index::try_new(id) {
        Ok(index) => Ok(Card::try_from_index(index).await?),
        Err(err) => Err(ServerFnError::ServerError {
            message: err.to_string(),
            code: 500,
            details: None,
        }),
    }
}

#[server(endpoint = "get_card_by_name_remote")]
pub async fn get_card_by_name_remote(name: String) -> Result<Card, ServerFnError> {
    info!("get card from remote with name: {name}");
    Ok(Card::try_from_name(Name::new(name.as_str())).await?)
}

#[server(endpoint = "get_card_by_id_db")]
pub async fn get_card_by_id_db(id: usize) -> Result<Card> {
    info!("get card from DB with id: {id}");
    DB.with(|db| {
        db.prepare(
            "SELECT id, name_en, name_de, book, page, side, entry, img_url, owned FROM cards WHERE id = ?",
        )?
        .query_row([id], |row| {
            Ok(Card {
                index: row.get(0)?,
                name_en: row.get(1)?,
                name_de: row.get(2)?,
                book: row.get(3)?,
                page: row.get(4)?,
                side: row.get(5)?,
                entry: row.get(6)?,
                img_url: row.get(7)?,
                owned: row.get(8)?,
            })
        })
        .map_err(|e| e.into())
    })
}

#[server(endpoint = "get_card_by_name_db")]
pub async fn get_card_by_name_db(name: String) -> Result<Card> {
    info!("get card from DB with name: {name}");
    DB.with(|db| {
        db.prepare(
            "SELECT id, name_en, name_de, book, page, side, entry, img_url, owned FROM cards WHERE name_de = ? COLLATE NOCASE OR name_en = ? COLLATE NOCASE",
        )?
        .query_row([&name, &name], |row| {
            Ok(Card {
                index: row.get(0)?,
                name_en: row.get(1)?,
                name_de: row.get(2)?,
                book: row.get(3)?,
                page: row.get(4)?,
                side: row.get(5)?,
                entry: row.get(6)?,
                img_url: row.get(7)?,
                owned: row.get(8)?,
            })
        })
        .map_err(|e| e.into())
    })
}

#[server(endpoint = "get_cards_with_timestamp_db")]
pub async fn get_cards_with_timestamp_db() -> Result<Vec<(Card, String)>> {
    info!("get all cards from DB");
    DB.with(|db| {
        Ok(db
            .prepare(
                "SELECT id, name_en, name_de, book, page, side, entry, img_url, owned, created_at FROM cards",
            )?
            .query_map([], |row| {
                Ok((
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
                    row.get(9)?
                ))
            })?
            .collect::<Result<Vec<(Card, String)>, rusqlite::Error>>()?)
    })
}

#[server(endpoint = "save_card_db")]
pub async fn save_card_db(card: Card) -> Result<(), ServerFnError> {
    info!("save card to DB: {card:#?}");
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
