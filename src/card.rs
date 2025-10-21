use anyhow::{anyhow, Result};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Card {
    pub index: Index,
    pub name_en: Name,
    pub name_de: Name,
    pub book: Book,
    pub page: Page,
    pub side: Side,
    pub entry: Entry,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Index(pub usize);

impl Index {
    pub fn try_new(index: usize) -> Result<Self> {
        if index == 0 {
            Err(anyhow!("Provided Card ID: {index} can't be lower than 1").into())
        } else {
            Ok(Self(index))
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Name(pub String);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Book(pub usize);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Page(pub usize);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Side {
    A,
    B,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Entry(pub usize);
