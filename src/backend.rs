use crate::card::Card;
use crate::expansion::{CardExpansion, Expansion};
use crate::{log_db_op, log_ownership_change, log_server_fn};
use anyhow::Result;
use dioxus::prelude::*;
use std::collections::HashMap;
#[cfg(feature = "dev")]
use std::fs;

#[cfg(feature = "server")]
thread_local! {
    static DB: std::sync::LazyLock<rusqlite::Connection> = std::sync::LazyLock::new(|| {
        #[cfg(not(feature = "dev"))]
        let conn = rusqlite::Connection::open("db/production.db").expect("Failed to open database");

        #[cfg(feature = "dev")]
        fs::create_dir_all("db_dev/").unwrap();
        #[cfg(feature = "dev")]
        let conn = rusqlite::Connection::open("db_dev/dev.db").expect("Failed to open database");

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
            );

            CREATE TABLE IF NOT EXISTS expansions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                abbreviation TEXT NOT NULL UNIQUE,
                cards INTEGER NOT NULL CHECK (cards > 0),
                secret_cards INTEGER NOT NULL CHECK (secret_cards >= 0)
            );

            CREATE TABLE IF NOT EXISTS card_expansions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                card_id INTEGER NOT NULL,
                expansion_id INTEGER NOT NULL,
                card_number TEXT NOT NULL,
                rarity TEXT NOT NULL,
                created_at DATETIME DEFAULT (datetime('now', 'localtime')),
                FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE,
                FOREIGN KEY (expansion_id) REFERENCES expansions(id) ON DELETE CASCADE,
                UNIQUE(card_id, expansion_id)
            );

            CREATE INDEX IF NOT EXISTS idx_card_expansions_card_id ON card_expansions(card_id);
            CREATE INDEX IF NOT EXISTS idx_card_expansions_expansion_id ON card_expansions(expansion_id);",
        )
        .expect("failed to create database tables");

        // Load expansions from SQL file if needed
        init_expansions_if_needed(&conn).expect("failed to initialize expansions");

        conn
    });
}

#[cfg(feature = "server")]
fn init_expansions_if_needed(conn: &rusqlite::Connection) -> Result<()> {
    // Check if expansions table is empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM expansions", [], |row| row.get(0))?;

    if count == 0 {
        tracing::info!("Loading expansions from SQL file");
        let sql_content = include_str!("../db_seed/expansions.sql");
        conn.execute_batch(sql_content)?;

        let new_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM expansions", [], |row| row.get(0))?;
        tracing::info!(count = new_count, "Loaded expansions");
    }

    Ok(())
}

#[server(endpoint = "validate_password")]
pub async fn validate_password(password: String) -> Result<bool, ServerFnError> {
    log_server_fn!("validate_password", password_length = password.len());

    let correct_password = std::env::var("APP_PASSWORD").unwrap();
    let is_valid = password == correct_password;

    if is_valid {
        tracing::info!("authentication successful");
    } else {
        tracing::warn!("authentication failed - incorrect password");
    }

    Ok(is_valid)
}

#[server(endpoint = "get_card_by_id_remote")]
pub async fn get_card_by_id_remote(id: usize) -> Result<Card, ServerFnError> {
    log_server_fn!("get_card_by_id_remote", card_id = id);
    use crate::card::Index;
    match Index::try_new(id) {
        Ok(index) => {
            let result = Card::try_from_index(index).await;
            match &result {
                Ok(card) => {
                    tracing::info!(card_id = id, name = %card.name_en.0, "fetched card from remote API")
                }
                Err(e) => {
                    tracing::error!(card_id = id, error = %e, "failed to fetch card from remote API")
                }
            }
            result.map_err(|e| e.into())
        }
        Err(err) => {
            tracing::error!(card_id = id, error = %err, "invalid card id");
            Err(ServerFnError::ServerError {
                message: err.to_string(),
                code: 500,
                details: None,
            })
        }
    }
}

#[server(endpoint = "get_card_by_name_remote")]
pub async fn get_card_by_name_remote(name: String) -> Result<Card, ServerFnError> {
    log_server_fn!("get_card_by_name_remote", name = &name);
    use crate::card::Name;
    let result = Card::try_from_name(Name::new(name.as_str())).await;
    match &result {
        Ok(card) => {
            tracing::info!(card_id = card.index.0, name = %card.name_en.0, "fetched card by name from remote API")
        }
        Err(e) => {
            tracing::error!(name = %name, error = %e, "failed to fetch card by name from remote API")
        }
    }
    result.map_err(|e| e.into())
}

#[server(endpoint = "get_card_by_id_db")]
pub async fn get_card_by_id_db(id: usize) -> Result<Card> {
    log_server_fn!("get_card_by_id_db", card_id = id);

    let result = DB.with(|db| {
        log_db_op!("SELECT", table = "cards", card_id = id);
        db.prepare(
            "SELECT id, name_en, name_de, book, page, side, entry, img_url, owned  FROM cards WHERE id = ?",
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
    });

    match &result {
        Ok(card) => {
            tracing::debug!(card_id = id, name = %card.name_en.0, "card fetched from database")
        }
        Err(e) => tracing::debug!(card_id = id, error = %e, "card not found in database"),
    }

    result
}

#[server(endpoint = "get_card_by_name_db")]
pub async fn get_card_by_name_db(name: String) -> Result<Card> {
    log_server_fn!("get_card_by_name_db", name = &name);

    let result = DB.with(|db| {
        log_db_op!("SELECT", table = "cards", name = &name);
        db.prepare(
            "SELECT id, name_en, name_de, book, page, side, entry, img_url, owned  FROM cards WHERE name_de = ? COLLATE NOCASE OR name_en = ? COLLATE NOCASE",
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
    });

    match &result {
        Ok(card) => {
            tracing::debug!(card_id = card.index.0, name = %card.name_en.0, "card fetched from database by name")
        }
        Err(e) => tracing::debug!(name = %name, error = %e, "card not found in database by name"),
    }

    result
}

#[server(endpoint = "get_cards_with_timestamp_db")]
pub async fn get_cards_with_timestamp_db() -> Result<Vec<(Card, String)>> {
    log_server_fn!("get_cards_with_timestamp_db");
    let result = DB.with(|db| {
        log_db_op!("SELECT", table = "cards", operation = "get_all_with_timestamp");
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
    });
    match &result {
        Ok(cards) => tracing::debug!(count = cards.len(), "fetched cards with timestamps"),
        Err(e) => tracing::error!(error = %e, "failed to fetch cards with timestamps"),
    }
    result
}

#[server(endpoint = "save_card_db")]
pub async fn save_card_db(card: Card) -> Result<(), ServerFnError> {
    use rusqlite::params;

    log_server_fn!("save_card_db", card_id = card.index.0, owned = card.owned.0);

    let result = DB.with(|f| {
        log_db_op!("INSERT OR REPLACE", table = "cards", card_id = card.index.0);
        f.execute(
            "INSERT OR REPLACE INTO cards (id, name_en, name_de, book, page, side, entry, img_url, owned) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![card.index, card.name_en, card.name_de, card.book, card.page, card.side, card.entry, card.img_url, card.owned],
        )
    });

    match result {
        Ok(_) => {
            tracing::info!(
                card_id = card.index.0,
                owned = card.owned.0,
                "card saved to database"
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!(card_id = card.index.0, error = %err, "failed to save card to database");
            Err(ServerFnError::ServerError {
                message: err.to_string(),
                code: 500,
                details: Some("could not save card to DB".into()),
            })
        }
    }
}

#[server(endpoint = "get_all_owned_cards_db")]
pub async fn get_all_owned_cards_db() -> Result<HashMap<usize, Card>, ServerFnError> {
    log_server_fn!("get_all_owned_cards_db");

    let result = DB.with(|db| {
        log_db_op!("SELECT", table = "cards", filter = "owned = 1");
        let mut stmt = db.prepare(
            "SELECT id, name_en, name_de, book, page, side, entry, img_url, owned  FROM cards WHERE owned = 1",
        )?;

        let cards = stmt
            .query_map([], |row| {
                let card = Card {
                    index: row.get(0)?,
                    name_en: row.get(1)?,
                    name_de: row.get(2)?,
                    book: row.get(3)?,
                    page: row.get(4)?,
                    side: row.get(5)?,
                    entry: row.get(6)?,
                    img_url: row.get(7)?,
                    owned: row.get(8)?,
                };
                Ok((card.index.0, card))
            })?
            .collect::<Result<HashMap<usize, Card>, rusqlite::Error>>()?;

        Ok(cards)
    });

    match &result {
        Ok(cards) => {
            tracing::info!(count = cards.len(), "fetched owned cards from database");
            result.map_err(|e: anyhow::Error| ServerFnError::ServerError {
                message: e.to_string(),
                code: 500,
                details: Some("could not fetch owned cards from DB".into()),
            })
        }
        Err(e) => {
            tracing::error!(error = %e, "failed to fetch owned cards from database");
            Err(ServerFnError::ServerError {
                message: e.to_string(),
                code: 500,
                details: Some("could not fetch owned cards from DB".into()),
            })
        }
    }
}

#[server(endpoint = "update_card_db")]
pub async fn update_card_db(card: Card) -> Result<(), ServerFnError> {
    use rusqlite::params;

    log_server_fn!(
        "update_card_db",
        card_id = card.index.0,
        owned = card.owned.0
    );

    // Track ownership change - fetch old state first
    let old_owned = DB.with(|db| {
        db.query_row(
            "SELECT owned FROM cards WHERE id = ?",
            [card.index.0],
            |row| row.get::<_, bool>(0),
        )
        .ok()
    });

    if let Some(old) = old_owned {
        if old != card.owned.0 {
            log_ownership_change!(card.index.0, old, card.owned.0);
        }
    }

    let result = DB.with(|f| {
        log_db_op!("UPDATE", table = "cards", card_id = card.index.0);
        f.execute(
            "UPDATE cards SET name_en = ?1, name_de = ?2, book = ?3, page = ?4, side = ?5, entry = ?6, img_url = ?7, owned = ?8  WHERE id = ?9",
            params![card.name_en, card.name_de, card.book, card.page, card.side, card.entry, card.img_url, card.owned, card.index],
        )
    });

    match result {
        Ok(_) => {
            tracing::info!(
                card_id = card.index.0,
                owned = card.owned.0,
                "card updated in database"
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!(card_id = card.index.0, error = %err, "failed to update card in database");
            Err(ServerFnError::ServerError {
                message: err.to_string(),
                code: 500,
                details: Some("could not update card in DB".into()),
            })
        }
    }
}

// ==================== Expansion Server Functions ====================

#[server(endpoint = "get_all_expansions_db")]
pub async fn get_all_expansions_db() -> Result<Vec<Expansion>, ServerFnError> {
    log_server_fn!("get_all_expansions_db");
    DB.with(|db| {
        let mut stmt = db.prepare(
            "SELECT id, name, abbreviation, cards, secret_cards FROM expansions ORDER BY name",
        )?;

        let expansions = stmt
            .query_map([], |row| {
                Ok(Expansion {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    abbreviation: row.get(2)?,
                    cards: row.get(3)?,
                    secret_cards: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<Expansion>, rusqlite::Error>>()?;

        Ok(expansions)
    })
    .map_err(|e: anyhow::Error| ServerFnError::ServerError {
        message: e.to_string(),
        code: 500,
        details: Some("could not fetch expansions from DB".into()),
    })
}

#[server(endpoint = "get_card_expansions_db")]
pub async fn get_card_expansions_db(card_id: usize) -> Result<Vec<CardExpansion>, ServerFnError> {
    log_server_fn!("get_card_expansions_db", card_id = card_id);
    DB.with(|db| {
        let mut stmt = db.prepare(
            "SELECT id, card_id, expansion_id, card_number, rarity FROM card_expansions WHERE card_id = ?",
        )?;

        let card_expansions = stmt
            .query_map([card_id], |row| {
                Ok(CardExpansion {
                    id: Some(row.get(0)?),
                    card_id: row.get(1)?,
                    expansion_id: row.get(2)?,
                    card_number: row.get(3)?,
                    rarity: row.get(4)?
                })
            })?
            .collect::<Result<Vec<CardExpansion>, rusqlite::Error>>()?;

        Ok(card_expansions)
    })
    .map_err(|e: anyhow::Error| ServerFnError::ServerError {
        message: e.to_string(),
        code: 500,
        details: Some("could not fetch card expansions from DB".into()),
    })
}

#[server(endpoint = "save_card_expansion_db")]
pub async fn save_card_expansion_db(card_expansion: CardExpansion) -> Result<(), ServerFnError> {
    use rusqlite::params;

    log_server_fn!(
        "save_card_expansion_db",
        card_id = card_expansion.card_id,
        expansion_id = card_expansion.expansion_id
    );
    DB.with(|f| {
        f.execute(
            "INSERT INTO card_expansions (card_id, expansion_id, card_number, rarity) VALUES (?1, ?2, ?3, ?4)",
            params![
                card_expansion.card_id,
                card_expansion.expansion_id,
                card_expansion.card_number,
                card_expansion.rarity
            ],
        )
    })
    .map_err(|err| ServerFnError::ServerError {
        message: err.to_string(),
        code: 500,
        details: Some("could not save card expansion to DB".into()),
    })?;
    Ok(())
}

#[server(endpoint = "update_card_expansion_db")]
pub async fn update_card_expansion_db(card_expansion: CardExpansion) -> Result<(), ServerFnError> {
    use rusqlite::params;

    log_server_fn!(
        "update_card_expansion_db",
        card_id = card_expansion.card_id,
        expansion_id = card_expansion.expansion_id
    );

    if card_expansion.id.is_none() {
        return Err(ServerFnError::ServerError {
            message: "Card expansion ID is required for update".to_string(),
            code: 400,
            details: None,
        });
    }

    DB.with(|f| {
        f.execute(
            "UPDATE card_expansions SET expansion_id = ?1, card_number = ?2, rarity = ?3 WHERE id = ?4",
            params![
                card_expansion.expansion_id,
                card_expansion.card_number,
                card_expansion.rarity,
                card_expansion.id.unwrap(),
            ],
        )
    })
    .map_err(|err| ServerFnError::ServerError {
        message: err.to_string(),
        code: 500,
        details: Some("could not update card expansion in DB".into()),
    })?;
    Ok(())
}

#[server(endpoint = "delete_card_expansion_db")]
pub async fn delete_card_expansion_db(id: usize) -> Result<(), ServerFnError> {
    use rusqlite::params;

    log_server_fn!("delete_card_expansion_db", expansion_id = id);
    tracing::info!(expansion_id = id, "deleting card expansion");
    DB.with(|f| f.execute("DELETE FROM card_expansions WHERE id = ?1", params![id]))
        .map_err(|err| ServerFnError::ServerError {
            message: err.to_string(),
            code: 500,
            details: Some("could not delete card expansion from DB".into()),
        })?;
    Ok(())
}

#[server(endpoint = "delete_all_card_expansions_db")]
pub async fn delete_all_card_expansions_db(card_id: usize) -> Result<(), ServerFnError> {
    use rusqlite::params;

    log_server_fn!("delete_all_card_expansions_db", card_id = card_id);
    tracing::info!(card_id = card_id, "deleting all expansions for card");
    DB.with(|f| {
        f.execute(
            "DELETE FROM card_expansions WHERE card_id = ?1",
            params![card_id],
        )
    })
    .map_err(|err| ServerFnError::ServerError {
        message: err.to_string(),
        code: 500,
        details: Some("could not delete card expansions from DB".into()),
    })?;
    Ok(())
}

// ==================== Client Logging ====================

/// Server function to log client-side errors
#[server(endpoint = "log_client_error")]
pub async fn log_client_error(
    component: String,
    message: String,
    details: Option<String>,
) -> Result<(), ServerFnError> {
    tracing::error!(
        component = %component,
        message = %message,
        details = ?details,
        source = "client",
        "client-side error"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Book, Entry, Index, Name, Page, Side};
    use serde::Serialize;
    use std::fs::File;
    use std::io::Write;

    #[derive(Serialize)]
    struct TestReport {
        total_pokemon: usize,
        success_count: usize,
        failed_count: usize,
        success_rate: f64,
        failed_indices: Vec<FailedTest>,
    }

    #[derive(Serialize)]
    struct FailedTest {
        id: usize,
        error_message: String,
        error_debug: String,
    }

    // ==================== Index Tests ====================

    #[test]
    fn test_index_valid_creation() {
        let index = Index::try_new(1);
        assert!(index.is_ok());
        assert_eq!(index.unwrap().0, 1);
    }

    #[test]
    fn test_index_zero_rejected() {
        let index = Index::try_new(0);
        assert!(index.is_err());
        assert!(index
            .unwrap_err()
            .to_string()
            .contains("can't be lower than 1"));
    }

    #[test]
    fn test_index_large_values() {
        let index = Index::try_new(10000);
        assert!(index.is_ok());
        assert_eq!(index.unwrap().0, 10000);
    }

    #[test]
    fn test_index_max_pokemon() {
        let index = Index::try_new(1025);
        assert!(index.is_ok());
        assert_eq!(index.unwrap().0, 1025);
    }

    // ==================== Book Tests ====================

    #[test]
    fn test_book_calculation_first_card() {
        let index = Index::try_new(1).unwrap();
        let book = Book::from(&index);
        assert_eq!(book.0, 1, "First card should be in book 1");
    }

    #[test]
    fn test_book_calculation_boundary() {
        // CARDS_PER_BOOK = 576
        let index = Index::try_new(576).unwrap();
        let book = Book::from(&index);
        assert_eq!(book.0, 1, "Card 576 should be in book 1");

        let index = Index::try_new(577).unwrap();
        let book = Book::from(&index);
        assert_eq!(book.0, 2, "Card 577 should be in book 2");
    }

    #[test]
    fn test_book_calculation_second_book() {
        let index = Index::try_new(600).unwrap();
        let book = Book::from(&index);
        assert_eq!(book.0, 2, "Card 600 should be in book 2");
    }

    #[test]
    fn test_book_calculation_large_index() {
        let index = Index::try_new(1025).unwrap();
        let book = Book::from(&index);
        // 1025 / 576 = 1.78... -> ceil = 2
        assert_eq!(book.0, 2, "Card 1025 should be in book 2");
    }

    // ==================== Page Tests ====================

    #[test]
    fn test_page_absolute_first_card() {
        let index = Index::try_new(1).unwrap();
        let page = Page::absolut(&index);
        assert_eq!(page.0, 1, "First card should be on absolute page 1");
    }

    #[test]
    fn test_page_absolute_boundary() {
        // CARDS_PER_PAGE = 24
        let index = Index::try_new(24).unwrap();
        let page = Page::absolut(&index);
        assert_eq!(page.0, 1, "Card 24 should be on absolute page 1");

        let index = Index::try_new(25).unwrap();
        let page = Page::absolut(&index);
        assert_eq!(page.0, 2, "Card 25 should be on absolute page 2");
    }

    #[test]
    fn test_page_relative_first_page() {
        let index = Index::try_new(1).unwrap();
        let page = Page::relative(&index);
        assert_eq!(page.0, 1, "First card should be on relative page 1");
    }

    #[test]
    fn test_page_relative_wrapping() {
        // CARDS_PER_BOOK = 576, CARDS_PER_PAGE = 24
        // Pages per book = 576 / 24 = 24
        // Card 576 is on page 24 of book 1
        let index = Index::try_new(576).unwrap();
        let page = Page::relative(&index);
        assert_eq!(page.0, 24, "Card 576 should be on page 24");

        // Card 577 starts book 2, should be page 1
        let index = Index::try_new(577).unwrap();
        let page = Page::relative(&index);
        assert_eq!(page.0, 1, "Card 577 should be on page 1 of book 2");
    }

    // ==================== Side Tests ====================

    #[test]
    fn test_side_first_half_of_page() {
        // First 12 cards of a page should be side A
        let index = Index::try_new(1).unwrap();
        let side = Side::from(&index);
        assert_eq!(side, Side::A, "Card 1 should be on side A");

        let index = Index::try_new(12).unwrap();
        let side = Side::from(&index);
        assert_eq!(side, Side::A, "Card 12 should be on side A");
    }

    #[test]
    fn test_side_second_half_of_page() {
        // Last 12 cards of a page should be side B
        let index = Index::try_new(13).unwrap();
        let side = Side::from(&index);
        assert_eq!(side, Side::B, "Card 13 should be on side B");

        let index = Index::try_new(24).unwrap();
        let side = Side::from(&index);
        assert_eq!(side, Side::B, "Card 24 should be on side B");
    }

    #[test]
    fn test_side_boundary_cases() {
        // Test page boundaries
        let index = Index::try_new(25).unwrap(); // First card of page 2
        let side = Side::from(&index);
        assert_eq!(side, Side::A, "Card 25 should be on side A");

        let index = Index::try_new(36).unwrap(); // 12th card of page 2
        let side = Side::from(&index);
        assert_eq!(side, Side::A, "Card 36 should be on side A");

        let index = Index::try_new(37).unwrap(); // 13th card of page 2
        let side = Side::from(&index);
        assert_eq!(side, Side::B, "Card 37 should be on side B");
    }

    // ==================== Entry Tests ====================

    #[test]
    fn test_entry_first_page_side_a() {
        let index = Index::try_new(1).unwrap();
        let page_abs = Page::absolut(&index);
        let side = Side::from(&index);
        let entry = Entry::new(&index, &page_abs, &side);
        assert_eq!(entry.0, 1, "First card should be entry 1");
    }

    #[test]
    fn test_entry_first_page_all_cards() {
        // Test all cards on first page
        for i in 1..=12 {
            let index = Index::try_new(i).unwrap();
            let page_abs = Page::absolut(&index);
            let side = Side::from(&index);
            let entry = Entry::new(&index, &page_abs, &side);
            assert_eq!(entry.0, i, "Card {} should have entry {}", i, i);
        }
    }

    #[test]
    fn test_entry_side_b_calculation() {
        // Cards 13-24 on page 1, side B
        let index = Index::try_new(13).unwrap();
        let page_abs = Page::absolut(&index);
        let side = Side::from(&index);
        let entry = Entry::new(&index, &page_abs, &side);
        // midpoint = 24 - 12 = 12
        // entry = 13 - 12 = 1
        assert_eq!(entry.0, 1, "Card 13 should be entry 1 on side B");
    }

    // ==================== Name Tests ====================

    #[test]
    fn test_name_creation() {
        let name = Name::new("pikachu");
        assert_eq!(name.0, "pikachu");
    }

    #[test]
    fn test_name_with_spaces() {
        let name = Name::new("mr mime");
        assert_eq!(name.0, "mr mime");
    }

    #[test]
    fn test_name_empty_string() {
        let name = Name::new("");
        assert_eq!(name.0, "");
    }

    #[test]
    fn test_name_unicode() {
        let name = Name::new("Flabébé");
        assert_eq!(name.0, "Flabébé");
    }

    #[test]
    fn test_name_special_characters() {
        let name = Name::new("Nidoran♀");
        assert_eq!(name.0, "Nidoran♀");
    }

    // ==================== Display Trait Tests ====================

    #[test]
    fn test_index_display() {
        let index = Index::try_new(123).unwrap();
        assert_eq!(format!("{}", index), "123");
    }

    #[test]
    fn test_name_display() {
        let name = Name::new("charizard");
        assert_eq!(format!("{}", name), "charizard");
    }

    #[test]
    fn test_book_display() {
        let index = Index::try_new(1).unwrap();
        let book = Book::from(&index);
        assert_eq!(format!("{}", book), "1");
    }

    #[test]
    fn test_side_display() {
        assert_eq!(format!("{}", Side::A), "A");
        assert_eq!(format!("{}", Side::B), "B");
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_very_large_index() {
        let index = Index::try_new(usize::MAX);
        assert!(index.is_ok());
    }

    #[test]
    fn test_card_calculations_consistency() {
        // Ensure that for any card, the calculations are consistent
        for id in [1, 24, 25, 576, 577, 1000, 1025].iter() {
            let index = Index::try_new(*id).unwrap();
            let book = Book::from(&index);
            let page_rel = Page::relative(&index);
            let page_abs = Page::absolut(&index);
            let side = Side::from(&index);
            let entry = Entry::new(&index, &page_abs, &side);

            // Book should be positive
            assert!(book.0 > 0, "Book must be > 0 for card {}", id);

            // Page should be positive
            assert!(page_rel.0 > 0, "Page must be > 0 for card {}", id);
            assert!(page_abs.0 > 0, "Absolute page must be > 0 for card {}", id);

            // Entry should be between 1 and 12
            assert!(
                entry.0 >= 1 && entry.0 <= 12,
                "Entry must be 1-12 for card {}, got {}",
                id,
                entry.0
            );
        }
    }

    /// Extracts the English name for a Pokemon from CSV content by ID
    fn extract_name_from_csv(csv_content: &str, pokemon_id: usize) -> Option<String> {
        use crate::csv_record;

        let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());
        for record in rdr.deserialize().flatten() {
            let record: csv_record::Record = record;
            // Look for the species and English language (ID 9)
            if record.pokemon_species_id == pokemon_id as u32 && record.local_language_id == 9 {
                return Some(record.name);
            }
        }
        None
    }

    #[tokio::test]
    #[ignore = "very slow integration test for all IDs"]
    async fn test_all_pokemon_by_id() {
        println!("\n=== Starting comprehensive Pokemon search test ===");
        println!("Testing all 1025 Pokemon IDs...\n");

        // Fetch CSV once at the beginning
        println!("Downloading CSV data for name extraction...");
        let csv_content = match reqwest::get(crate::LANGUAGE_URL).await {
            Ok(resp) => match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Failed to read CSV content: {}", e);
                    String::new()
                }
            },
            Err(e) => {
                eprintln!("Failed to download CSV: {}", e);
                String::new()
            }
        };
        let has_csv = !csv_content.is_empty();
        println!(
            "CSV download: {}\n",
            if has_csv { "✓ Success" } else { "✗ Failed" }
        );

        let mut failed_indices = Vec::new();
        let mut success_count = 0;
        let total_pokemon = 1025;
        let mut auto_lut: std::collections::HashMap<usize, String> =
            std::collections::HashMap::new();

        for id in 1..=total_pokemon {
            // Print progress every 50 Pokemon
            if id % 50 == 0 {
                println!("Progress: {}/{} Pokemon tested...", id, total_pokemon);
            }

            // Test creating an Index and fetching the Card
            match Index::try_new(id) {
                Ok(index) => {
                    match Card::try_from_index(index).await {
                        Ok(card) => {
                            success_count += 1;
                            // Optionally print successful fetches (commented out to reduce noise)
                            println!("✓ ID {}: {} ({})", id, card.name_en, card.name_de);
                        }
                        Err(err) => {
                            let error_msg = format!("Card creation failed - {}", err);
                            let error_debug = format!("{:?}", err);
                            println!("✗ ID {}: {}", id, error_msg);

                            // Try to extract the name from CSV for auto-LUT
                            if has_csv {
                                if let Some(csv_name) = extract_name_from_csv(&csv_content, id) {
                                    auto_lut.insert(id, csv_name.clone());
                                    println!("  → Auto-LUT: \"{}\" -> \"{}\"", id, csv_name);
                                }
                            }

                            failed_indices.push((id, error_msg, error_debug));
                        }
                    }
                }
                Err(err) => {
                    let error_msg = format!("Index validation failed - {}", err);
                    let error_debug = format!("{:?}", err);
                    println!("✗ ID {}: {}", id, error_msg);
                    failed_indices.push((id, error_msg, error_debug));
                }
            }

            // Small delay to avoid hammering the API too hard
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        // Print final summary
        println!("\n=== Test Summary ===");
        println!("Total Pokemon tested: {}", total_pokemon);
        println!("Successful: {}", success_count);
        println!("Failed: {}", failed_indices.len());

        if !failed_indices.is_empty() {
            println!("\n=== Failed Indices ===");
            for (id, error, _) in &failed_indices {
                println!("  - ID {}: {}", id, error);
            }

            println!("\n=== Failed IDs (comma-separated) ===");
            let failed_ids: Vec<String> = failed_indices
                .iter()
                .map(|(id, _, _)| id.to_string())
                .collect();
            println!("[{}]", failed_ids.join(", "));
        } else {
            println!("\nAll Pokemon IDs fetched successfully!");
        }

        // Create JSON report
        let success_rate = (success_count as f64 / total_pokemon as f64) * 100.0;
        let report = TestReport {
            total_pokemon,
            success_count,
            failed_count: failed_indices.len(),
            success_rate,
            failed_indices: failed_indices
                .iter()
                .map(|(id, error_message, error_debug)| FailedTest {
                    id: *id,
                    error_message: error_message.clone(),
                    error_debug: error_debug.clone(),
                })
                .collect(),
        };

        // Write JSON report to file
        let json_output =
            serde_json::to_string_pretty(&report).expect("Failed to serialize report");
        let mut file = File::create("test_report.json").expect("Failed to create report file");
        file.write_all(json_output.as_bytes())
            .expect("Failed to write report");

        println!("\nTest report saved to: test_report.json");

        // Generate auto-populated LUT from CSV names
        if !auto_lut.is_empty() {
            let mut lut_data = serde_json::Map::new();
            // Sort by ID for easier reading
            let mut sorted_ids: Vec<_> = auto_lut.keys().collect();
            sorted_ids.sort();

            for id in sorted_ids {
                lut_data.insert(
                    id.to_string(),
                    serde_json::Value::String(auto_lut[id].clone()),
                );
            }

            let lut_json =
                serde_json::to_string_pretty(&lut_data).expect("Failed to serialize auto-LUT");
            let mut lut_file = File::create("pokemon_name_overrides_auto.json")
                .expect("Failed to create auto-LUT file");
            lut_file
                .write_all(lut_json.as_bytes())
                .expect("Failed to write auto-LUT");

            println!("\n📝 Auto-populated LUT saved to: pokemon_name_overrides_auto.json");
            println!("   Contains {} entries extracted from CSV", auto_lut.len());

            // Automatically copy to active LUT file
            match std::fs::copy(
                "pokemon_name_overrides_auto.json",
                "pokemon_name_overrides.json",
            ) {
                Ok(_) => {
                    println!("✅ Automatically copied to pokemon_name_overrides.json (active)");
                }
                Err(e) => {
                    println!("⚠️  Failed to copy to active file: {}", e);
                    println!("   Manual copy required: cp pokemon_name_overrides_auto.json pokemon_name_overrides.json");
                }
            }
        }

        // Assert that we have a reasonable success rate
        // You can adjust this threshold based on your expectations
        assert!(
            success_count > 1000,
            "Too many failures: only {}/{} succeeded",
            success_count,
            total_pokemon
        );
    }
}
