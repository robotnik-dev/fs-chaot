#[cfg(feature = "server")]
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};
use std::fmt::Display;

use crate::card::Rarity;

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Expansion {
    pub id: usize,
    pub name: String,
    pub abbreviation: String,
    pub cards: usize,
    pub secret_cards: usize,
}

impl Display for Expansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.abbreviation)
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CardExpansion {
    pub id: Option<usize>, // None for new entries (auto-increment)
    pub card_id: usize,
    pub expansion_id: usize,
    pub card_number: String,
    pub rarity: Rarity,
}

impl CardExpansion {
    /// Calculates if a card is secret based on card_number and expansion.cards
    /// Returns true if card_number (as integer) > expansion.cards
    #[allow(dead_code)]
    pub fn is_secret(&self, expansion: &Expansion) -> bool {
        self.card_number
            .parse::<usize>()
            .map(|num| num > expansion.cards)
            .unwrap_or(false) // Non-numeric card numbers (promos) default to false
    }

    /// Formats card number for display: "24/165" format
    #[allow(dead_code)]
    pub fn display_card_number(&self, expansion: &Expansion) -> String {
        format!("{}/{}", self.card_number, expansion.cards)
    }
}

// Server-side serialization for Expansion
#[cfg(feature = "server")]
impl ToSql for Expansion {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        // Expansions are stored as individual fields, this is for completeness
        Err(rusqlite::Error::ToSqlConversionFailure(
            "Expansion should be serialized field by field".into(),
        ))
    }
}

#[cfg(feature = "server")]
impl FromSql for Expansion {
    fn column_result(_value: ValueRef<'_>) -> FromSqlResult<Self> {
        // Expansions are loaded field by field from SQL, this is for completeness
        Err(rusqlite::types::FromSqlError::InvalidType)
    }
}

// Server-side serialization for CardExpansion
#[cfg(feature = "server")]
impl ToSql for CardExpansion {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        // CardExpansions are stored as individual fields, this is for completeness
        Err(rusqlite::Error::ToSqlConversionFailure(
            "CardExpansion should be serialized field by field".into(),
        ))
    }
}

#[cfg(feature = "server")]
impl FromSql for CardExpansion {
    fn column_result(_value: ValueRef<'_>) -> FromSqlResult<Self> {
        // CardExpansions are loaded field by field from SQL, this is for completeness
        Err(rusqlite::types::FromSqlError::InvalidType)
    }
}
