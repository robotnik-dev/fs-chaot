use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::{
    card::{Index, Name},
    csv_record,
};

/// Global cache for Pokemon name overrides (for CSV lookups)
static NAME_OVERRIDES: OnceLock<HashMap<usize, String>> = OnceLock::new();

/// Loads the Pokemon name override lookup table from the JSON file
fn load_name_overrides() -> HashMap<usize, String> {
    match std::fs::read_to_string("pokemon_name_overrides.json") {
        Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(json) => {
                let mut overrides = HashMap::new();
                if let Some(obj) = json.as_object() {
                    for (key, value) in obj {
                        // Skip comments and examples
                        if key.starts_with('_') {
                            continue;
                        }
                        if let Ok(id) = key.parse::<usize>() {
                            if let Some(name) = value.as_str() {
                                overrides.insert(id, name.to_string());
                            }
                        }
                    }
                }
                overrides
            }
            Err(_) => HashMap::new(),
        },
        Err(_) => HashMap::new(),
    }
}

/// Gets the name override lookup table (loads once, then caches)
fn get_name_overrides() -> &'static HashMap<usize, String> {
    NAME_OVERRIDES.get_or_init(load_name_overrides)
}

#[derive(Debug, Clone)]
pub struct PokeApi;

impl PokeApi {
    // Language IDs in the CSV data
    const GERMAN_LANG_ID: u8 = 6;
    const ENGLISH_LANG_ID: u8 = 9;

    pub async fn get_id(base_url: &str, language_url: &str, name: &Name) -> Result<usize> {
        let name_normalized = name.0.replace(" ", "-");

        // Try to fetch directly with the provided name (assuming English)
        match PokeApi::fetch_pokemon_json(base_url, &name_normalized).await {
            Ok(value) => {
                let id = value["id"].to_string().parse::<usize>()?;
                Ok(id)
            }
            Err(_) => {
                // Name not found, try looking it up in CSV (might be German name)
                let csv_content = PokeApi::make_reqwest(language_url).await?;

                // Find species ID from the provided name
                let species_id = PokeApi::find_species_id_by_name(&csv_content, &name_normalized)?
                    .ok_or_else(|| {
                        anyhow!(
                            "No german or english version found for name: {}",
                            name_normalized
                        )
                    })?;

                // Get the English name for this species
                let english_name = PokeApi::find_name_by_species_id(
                    &csv_content,
                    species_id,
                    Self::ENGLISH_LANG_ID,
                )?
                .ok_or_else(|| {
                    anyhow!(
                        "No english CSV record found with species ID: {}",
                        species_id
                    )
                })?;

                // Fetch the pokemon data using the English name
                let english_name_normalized = english_name.replace(" ", "-");
                let value = PokeApi::fetch_pokemon_json(base_url, &english_name_normalized).await?;
                let id = value["id"].to_string().parse::<usize>()?;
                Ok(id)
            }
        }
    }

    pub async fn get_names(
        index: &Index,
        base_url: &str,
        language_url: &str,
    ) -> Result<Vec<String>> {
        // Fetch English name from the API
        let url = format!("{}{}/", base_url, index.0);
        let resp = PokeApi::make_reqwest(&url).await?;
        if resp.to_lowercase() == "not found" {
            return Err(anyhow!("Nothing found for id: {} at {}", index, url));
        }

        let value: Value = serde_json::from_str(&resp)?;
        let english_name = value["species"]["name"]
            .to_string()
            .replace("\"", "")
            .to_lowercase();

        // Fetch CSV data to get the German name
        let csv_content = PokeApi::make_reqwest(language_url).await?;

        // Find the species ID for this English name
        let species_id = match PokeApi::find_species_id_by_name(&csv_content, &english_name)? {
            Some(id) => id,
            None => {
                // CSV lookup failed, check if we have an override for this Pokemon ID
                let overrides = get_name_overrides();
                if let Some(override_name) = overrides.get(&index.0) {
                    // Try with the override name from the LUT
                    match PokeApi::find_species_id_by_name(&csv_content, override_name)? {
                        Some(id) => id,
                        None => {
                            return Err(anyhow!(
                                "Couldn't find name in csv file (tried '{}' and override '{}'): URL: {}",
                                english_name,
                                override_name,
                                language_url
                            ));
                        }
                    }
                } else {
                    return Err(anyhow!(
                        "Couldn't find english name in csv file: URL: {}\nName: `{}`",
                        language_url,
                        english_name
                    ));
                }
            }
        };

        // Get the German name for this species
        let german_name =
            PokeApi::find_name_by_species_id(&csv_content, species_id, Self::GERMAN_LANG_ID)?
                .ok_or_else(|| {
                    anyhow!(
                "Couldn't find german name in csv file: URL: {}\nspecies ID: {}\nenglish name: {}",
                language_url,
                species_id,
                english_name
            )
                })?;

        Ok(vec![english_name, german_name.to_lowercase()])
    }

    async fn make_reqwest(url: &str) -> Result<String> {
        reqwest::get(url)
            .await
            .with_context(|| format!("Couldn't reach URL: {}", url))?
            .text()
            .await
            .context("Couldn't fetch text from URL")
    }

    /// Searches CSV data for a pokemon species ID by name (case-insensitive)
    fn find_species_id_by_name(csv_content: &str, search_name: &str) -> Result<Option<u32>> {
        let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());
        for result in rdr.deserialize() {
            let record: csv_record::Record = result.context("Couldn't parse csv record")?;
            if search_name.to_lowercase() == record.name.to_lowercase() {
                return Ok(Some(record.pokemon_species_id));
            }
        }
        Ok(None)
    }

    /// Finds a pokemon name in the CSV data by species ID and language ID
    fn find_name_by_species_id(
        csv_content: &str,
        species_id: u32,
        language_id: u8,
    ) -> Result<Option<String>> {
        let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());
        for result in rdr.deserialize() {
            let record: csv_record::Record = result.context("Couldn't parse csv record")?;
            if record.pokemon_species_id == species_id && record.local_language_id == language_id {
                return Ok(Some(record.name));
            }
        }
        Ok(None)
    }

    /// Fetches pokemon data from the API and parses the JSON response
    async fn fetch_pokemon_json(base_url: &str, name: &str) -> Result<Value> {
        let url = format!("{}{}/", base_url, name);
        let resp = PokeApi::make_reqwest(&url).await?;
        if resp.to_lowercase() == "not found" {
            return Err(anyhow!("Pokemon not found at: {}", url));
        }
        serde_json::from_str(&resp).context("Couldn't parse URL response into JSON")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== CSV Helper Tests ====================

    #[test]
    fn test_find_species_id_by_name_found() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n1,9,bulbasaur,Seed Pokémon\n1,6,bisasam,Samen-Pokémon\n25,9,pikachu,Mouse Pokémon";

        let result = PokeApi::find_species_id_by_name(csv_data, "pikachu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(25));
    }

    #[test]
    fn test_find_species_id_by_name_case_insensitive() {
        let csv_data =
            "pokemon_species_id,local_language_id,name,genus\n25,9,pikachu,Mouse Pokémon";

        let result = PokeApi::find_species_id_by_name(csv_data, "PIKACHU");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(25));

        let result = PokeApi::find_species_id_by_name(csv_data, "PiKaChU");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(25));
    }

    #[test]
    fn test_find_species_id_by_name_not_found() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n1,9,bulbasaur,Seed Pokémon\n25,9,pikachu,Mouse Pokémon";

        let result = PokeApi::find_species_id_by_name(csv_data, "charizard");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_find_species_id_by_name_empty_csv() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n";

        let result = PokeApi::find_species_id_by_name(csv_data, "pikachu");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_find_species_id_by_name_german() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n1,9,bulbasaur,Seed Pokémon\n1,6,bisasam,Samen-Pokémon";

        let result = PokeApi::find_species_id_by_name(csv_data, "bisasam");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1));
    }

    #[test]
    fn test_find_name_by_species_id_english() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n1,9,bulbasaur,Seed Pokémon\n1,6,bisasam,Samen-Pokémon\n25,9,pikachu,Mouse Pokémon";

        let result = PokeApi::find_name_by_species_id(csv_data, 1, PokeApi::ENGLISH_LANG_ID);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("bulbasaur".to_string()));
    }

    #[test]
    fn test_find_name_by_species_id_german() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n1,9,bulbasaur,Seed Pokémon\n1,6,bisasam,Samen-Pokémon\n25,9,pikachu,Mouse Pokémon";

        let result = PokeApi::find_name_by_species_id(csv_data, 1, PokeApi::GERMAN_LANG_ID);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("bisasam".to_string()));
    }

    #[test]
    fn test_find_name_by_species_id_not_found() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n1,9,bulbasaur,Seed Pokémon\n1,6,bisasam,Samen-Pokémon";

        let result = PokeApi::find_name_by_species_id(csv_data, 999, PokeApi::ENGLISH_LANG_ID);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_find_name_by_species_id_wrong_language() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n1,9,bulbasaur,Seed Pokémon\n1,6,bisasam,Samen-Pokémon";

        // Species 1 exists but not with language ID 5 (French)
        let result = PokeApi::find_name_by_species_id(csv_data, 1, 5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_find_name_by_species_id_empty_csv() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n";

        let result = PokeApi::find_name_by_species_id(csv_data, 1, PokeApi::ENGLISH_LANG_ID);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_find_species_id_with_special_characters() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n29,9,nidoran♀,Poison Pin Pokémon\n669,9,flabébé,Single Bloom Pokémon";

        let result = PokeApi::find_species_id_by_name(csv_data, "nidoran♀");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(29));

        let result = PokeApi::find_species_id_by_name(csv_data, "flabébé");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(669));
    }

    #[test]
    fn test_find_species_id_with_hyphen() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n122,9,mr-mime,Barrier Pokémon\n439,9,mime-jr,Mime Pokémon";

        let result = PokeApi::find_species_id_by_name(csv_data, "mr-mime");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(122));

        let result = PokeApi::find_species_id_by_name(csv_data, "mime-jr");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(439));
    }

    #[test]
    fn test_malformed_csv_error_handling() {
        // CSV with missing columns
        let csv_data = "pokemon_species_id,local_language_id\n1,9\n";

        let result = PokeApi::find_species_id_by_name(csv_data, "bulbasaur");
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_same_species_different_languages() {
        let csv_data = "pokemon_species_id,local_language_id,name,genus\n1,1,フシギダネ,たねポケモン\n1,5,bulbizarre,Pokémon Graine\n1,6,bisasam,Samen-Pokémon\n1,9,bulbasaur,Seed Pokémon";

        // Find species ID by any language
        let result = PokeApi::find_species_id_by_name(csv_data, "bulbasaur");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1));

        let result = PokeApi::find_species_id_by_name(csv_data, "bisasam");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1));

        // Get name by specific language
        let result = PokeApi::find_name_by_species_id(csv_data, 1, 5); // French
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("bulbizarre".to_string()));
    }

    // ==================== Language Constants Tests ====================

    #[test]
    fn test_language_constants() {
        assert_eq!(PokeApi::GERMAN_LANG_ID, 6);
        assert_eq!(PokeApi::ENGLISH_LANG_ID, 9);
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_empty_name_search() {
        let csv_data =
            "pokemon_species_id,local_language_id,name,genus\n1,9,bulbasaur,Seed Pokémon";

        let result = PokeApi::find_species_id_by_name(csv_data, "");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_whitespace_in_names() {
        let csv_data =
            "pokemon_species_id,local_language_id,name,genus\n122,9,mr mime,Barrier Pokémon";

        let result = PokeApi::find_species_id_by_name(csv_data, "mr mime");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(122));
    }

    #[test]
    fn test_csv_with_quotes() {
        let csv_data = r#"pokemon_species_id,local_language_id,name,genus
1,9,"bulbasaur","Seed Pokémon"
25,9,"pikachu","Mouse Pokémon""#;

        let result = PokeApi::find_species_id_by_name(csv_data, "bulbasaur");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1));
    }
}
