use crate::{
    pokeapi::PokeApi, BASE_URL, CARDS_PER_BOOK, CARDS_PER_DOUBLE_PAGE, LANGUAGE_URL, SPRITE_URL,
};
use anyhow::{anyhow, Result};
#[cfg(feature = "server")]
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput},
    ToSql,
};
use std::fmt::Display;
use strum::EnumIter;

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Card {
    pub index: Index,
    pub name_en: Name,
    pub name_de: Name,
    pub book: Book,
    pub page: Page,
    pub side: Side,
    pub entry: Entry,
    pub img_url: String,
    pub owned: Bool,
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
            ..Default::default()
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
            ..Default::default()
        })
    }

    pub async fn try_from_name(name: Name) -> Result<Self> {
        let id = PokeApi::get_id(BASE_URL, LANGUAGE_URL, &name).await?;
        let index = Index::try_new(id)?;
        Card::try_from_index(index).await
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    EnumIter,
    Ord,
    Eq,
    PartialOrd,
)]
pub enum Rarity {
    #[default]
    Common,
    Uncommon,
    Rare,
    HoloRare,
    ReverseHoloRare,
    DoubleRare,
    UltraRare,
    SecretRare,
    Promo,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Common => f.write_str("Common ●"),
            Self::Uncommon => f.write_str("Uncommon ♦"),
            Self::Rare => f.write_str("Rare ★"),
            Self::HoloRare => f.write_str("Holo Rare ★H"),
            Self::ReverseHoloRare => f.write_str("Reverse Holo Rare ★H"),
            Self::DoubleRare => f.write_str("Double Rare ★★"),
            Self::UltraRare => f.write_str("Ultra Rare"),
            Self::SecretRare => f.write_str("Secret Rare"),
            Self::Promo => f.write_str("Promo"),
        }
    }
}

// impl FromStr for Rarity {
//     type Err = Error;
//     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//         match s {
//             "Common ●" => Ok(Self::Common),
//             "Uncommon ♦" => Ok(Self::Uncommon),
//             "Rare ★" => Ok(Self::Rare),
//             "Holo Rare ★H" => Ok(Self::HoloRare),
//             "Reverse Holo Rare ★H" => Ok(Self::ReverseHoloRare),
//             "Double Rare ★★" => Ok(Self::DoubleRare),
//             "Ultra Rare" => Ok(Self::UltraRare),
//             "Secret Rare" => Ok(Self::SecretRare),
//             "Promo" => Ok(Self::Promo),
//             _ => Ok(Self::Common),
//         }
//     }
// }

#[cfg(feature = "server")]
impl ToSql for Rarity {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(
            self.to_string(),
        )))
    }
}

#[cfg(feature = "server")]
impl FromSql for Rarity {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        FromSqlResult::Ok(Rarity::from(value.as_str()?))
    }
}

impl From<&str> for Rarity {
    fn from(value: &str) -> Self {
        match value {
            "Common ●" => Self::Common,
            "Uncommon ♦" => Self::Uncommon,
            "Rare ★" => Self::Rare,
            "Holo Rare ★H" => Self::HoloRare,
            "Reverse Holo Rare ★H" => Self::ReverseHoloRare,
            "Double Rare ★★" => Self::DoubleRare,
            "Ultra Rare" => Self::UltraRare,
            "Secret Rare" => Self::SecretRare,
            "Promo" => Self::Promo,
            _ => Self::Common,
        }
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
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

#[cfg(feature = "server")]
impl ToSql for Index {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            self.0 as i64,
        )))
    }
}

#[cfg(feature = "server")]
impl FromSql for Index {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        FromSqlResult::Ok(Index(value.as_i64()? as usize))
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "server")]
impl ToSql for Name {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(
            self.to_string(),
        )))
    }
}

#[cfg(feature = "server")]
impl FromSql for Name {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        FromSqlResult::Ok(Name(value.as_str()?.to_string()))
    }
}

impl Name {
    pub fn new(name: &str) -> Self {
        Name(name.to_string())
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Book(pub usize);

#[cfg(feature = "server")]
impl ToSql for Book {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            self.0 as i64,
        )))
    }
}

#[cfg(feature = "server")]
impl FromSql for Book {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        FromSqlResult::Ok(Book(value.as_i64()? as usize))
    }
}

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

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Page(pub usize);

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

#[cfg(feature = "server")]
impl ToSql for Page {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            self.0 as i64,
        )))
    }
}

#[cfg(feature = "server")]
impl FromSql for Page {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        FromSqlResult::Ok(Page(value.as_i64()? as usize))
    }
}

impl Page {
    pub fn relative_from_absolute(absolute: Self) -> Self {
        let pages = CARDS_PER_BOOK / CARDS_PER_DOUBLE_PAGE;
        let remainder = absolute.0 % pages;
        if remainder == 0 {
            Self(CARDS_PER_DOUBLE_PAGE)
        } else {
            Self(remainder)
        }
    }

    /// Calculates the absolut page number counting from 0
    pub fn absolut(index: &Index) -> Self {
        Self((index.0 as f32 / CARDS_PER_DOUBLE_PAGE as f32).ceil() as usize)
    }

    /// Takes into the maximum cards per book into account and calculates the page relative to each book
    pub fn relative(index: &Index) -> Self {
        let pages = (CARDS_PER_BOOK / CARDS_PER_DOUBLE_PAGE) as u16;
        let page = (index.0 as f32 / CARDS_PER_DOUBLE_PAGE as f32).ceil() as u16;
        let remainder = page % pages;
        if remainder == 0 {
            Self(CARDS_PER_DOUBLE_PAGE)
        } else {
            Self(remainder as usize)
        }
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Side {
    #[default]
    A,
    B,
}

impl Side {
    pub fn from_page(page: Page) -> Self {
        if page.0.is_multiple_of(2) {
            Self::B
        } else {
            Self::A
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => f.write_str("A"),
            Self::B => f.write_str("B"),
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for Side {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(
            self.to_string(),
        )))
    }
}

#[cfg(feature = "server")]
impl FromSql for Side {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        FromSqlResult::Ok(Side::from(value.as_str()?))
    }
}

impl From<&str> for Side {
    fn from(value: &str) -> Self {
        if value == "A" {
            Self::A
        } else {
            Self::B
        }
    }
}

impl From<&Index> for Side {
    fn from(value: &Index) -> Self {
        let rest = (value.0 as f32 / CARDS_PER_DOUBLE_PAGE as f32).fract();
        if rest > 0.5 || rest == 0.0 {
            Self::B
        } else {
            Self::A
        }
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Entry(pub usize);

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

#[cfg(feature = "server")]
impl ToSql for Entry {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            self.0 as i64,
        )))
    }
}

#[cfg(feature = "server")]
impl FromSql for Entry {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        FromSqlResult::Ok(Entry(value.as_i64()? as usize))
    }
}

impl Entry {
    pub fn new(index: &Index, page_absolut: &Page, side: &Side) -> Self {
        let max_card_no = CARDS_PER_DOUBLE_PAGE * page_absolut.0;
        let midpoint = max_card_no - (CARDS_PER_DOUBLE_PAGE / 2);
        match side {
            Side::A => {
                if page_absolut.0 == 1 {
                    Self(index.0)
                } else {
                    Self((CARDS_PER_DOUBLE_PAGE / 2) - (midpoint % index.0))
                }
            }
            Side::B => Self(index.0 - midpoint),
        }
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Bool(pub bool);

impl Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            f.write_str("Yes")
        } else {
            f.write_str("No")
        }
    }
}

#[cfg(feature = "server")]
impl FromSql for Bool {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        use rusqlite::types::FromSqlError;

        let as_int = value.as_i64()?;
        match as_int {
            0 => Ok(Bool(false)),
            1 => Ok(Bool(true)),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for Bool {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
            if self.0 { 1 } else { 0 },
        )))
    }
}
