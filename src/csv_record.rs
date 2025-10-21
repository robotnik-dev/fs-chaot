#[derive(Debug, serde::Deserialize)]
pub(crate) struct Record {
    pub(crate) pokemon_species_id: u32,
    pub(crate) local_language_id: u8,
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) genus: String,
}
