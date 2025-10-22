use std::fmt::Display;

use anyhow::{anyhow, Result};

use crate::{pokeapi::PokeApi, BASE_URL, CARDS_PER_BOOK, CARDS_PER_PAGE, LANGUAGE_URL, SPRITE_URL};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Card {
    pub index: Index,
    pub name_en: Name,
    pub name_de: Name,
    pub book: Book,
    pub page: Page,
    pub side: Side,
    pub entry: Entry,
    pub img_url: String,
}

impl Card {
    #[allow(dead_code)]
    pub fn new_debug() -> Self {
        Card {
            index: Index(1),
            name_en: Name::new("some name"),
            name_de: Name::new("ein Name"),
            book: Book(1),
            page: Page(1),
            side: Side::A,
            entry: Entry(1),
            img_url: format!("{SPRITE_URL}1.png"),
        }
    }
    pub async fn try_from_index(index: Index) -> Result<Self> {
        let names = PokeApi::get_names(&index, BASE_URL, LANGUAGE_URL).await?;
        let name_en = Name::new(names[0].as_str());
        let name_de = Name::new(names[1].as_str());
        let book = Book::from(&index);
        let page = Page::relative(&index);
        let side = Side::from(&index);
        let entry = Entry::new(&index, &Page::absolut(&index), &side);
        let img_url = format!("{SPRITE_URL}{}.png", &index.0);
        Ok(Self {
            index,
            name_en,
            name_de,
            book,
            page,
            side,
            entry,
            img_url,
        })
    }

    pub async fn try_from_name(name: Name) -> Result<Self> {
        let id = PokeApi::get_id(BASE_URL, LANGUAGE_URL, &name).await?;
        let index = Index::try_new(id)?;
        Card::try_from_index(index).await
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Index(pub usize);

impl Index {
    pub fn try_new(index: usize) -> Result<Self> {
        if index == 0 {
            Err(anyhow!("Provided Card ID: {index} can't be lower than 1"))
        } else {
            Ok(Self(index))
        }
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Name {
    pub fn new(name: &str) -> Self {
        Name(name.to_string())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Book(pub usize);

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<&Index> for Book {
    fn from(value: &Index) -> Self {
        Book((value.0 as f32 / CARDS_PER_BOOK as f32).ceil() as usize)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Page(pub usize);

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl Page {
    /// Calculates the absolut page number counting from 0
    pub fn absolut(index: &Index) -> Self {
        Self((index.0 as f32 / CARDS_PER_PAGE as f32).ceil() as usize)
    }

    /// Takes into the maximum cards per book into account and calculates the page relative to each book
    pub fn relative(index: &Index) -> Self {
        let pages = (CARDS_PER_BOOK / CARDS_PER_PAGE) as u16;
        let page = (index.0 as f32 / CARDS_PER_PAGE as f32).ceil() as u16;
        let remainder = page % pages;
        if remainder == 0 {
            Self(CARDS_PER_PAGE)
        } else {
            Self(remainder as usize)
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Side {
    A,
    B,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => f.write_str("A"),
            Self::B => f.write_str("B"),
        }
    }
}

impl From<&Index> for Side {
    fn from(value: &Index) -> Self {
        let rest = (value.0 as f32 / CARDS_PER_PAGE as f32).fract();
        if rest > 0.5 || rest == 0.0 {
            Self::B
        } else {
            Self::A
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Entry(pub usize);

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl Entry {
    pub fn new(index: &Index, page_absolut: &Page, side: &Side) -> Self {
        let max_card_no = CARDS_PER_PAGE * page_absolut.0;
        let midpoint = max_card_no - (CARDS_PER_PAGE / 2);
        match side {
            Side::A => {
                if page_absolut.0 == 1 {
                    Self(index.0)
                } else {
                    Self((CARDS_PER_PAGE / 2) - (midpoint % index.0))
                }
            }
            Side::B => Self(index.0 - midpoint),
        }
    }
}
