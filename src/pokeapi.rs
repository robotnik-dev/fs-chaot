use anyhow::{anyhow, Context, Result};
use serde_json::Value;

use crate::{
    card::{Index, Name},
    csv_record,
};

#[derive(Debug, Clone)]
pub struct PokeApi;

impl PokeApi {
    pub async fn get_id(base_url: &str, language_url: &str, name: &Name) -> Result<usize> {
        let name = name.0.replace(" ", "-");
        let url = format!("{}{}/", base_url, name);
        let resp = PokeApi::make_reqwest(&url).await?;
        if resp.to_lowercase() == "not found" {
            // try german name
            let resp = PokeApi::make_reqwest(language_url).await?;
            // get csv data
            let mut pkm_species_id = None;
            let mut rdr = csv::Reader::from_reader(resp.as_bytes());
            for result in rdr.deserialize() {
                let record: csv_record::Record = result.context("Couldn't parse csv record")?;
                if name.to_lowercase() == record.name.to_lowercase() {
                    pkm_species_id = Some(record.pokemon_species_id);
                }
            }
            match pkm_species_id {
                Some(id) => {
                    let mut rdr = csv::Reader::from_reader(resp.as_bytes());
                    for result in rdr.deserialize() {
                        let record: csv_record::Record =
                            result.context("Couldn't parse csv record")?;
                        if id == record.pokemon_species_id && record.local_language_id == 9 {
                            let eng_name = record.name.replace(" ", "-");
                            let url = format!("{}{}/", base_url, eng_name);
                            let resp = PokeApi::make_reqwest(&url).await?;
                            let value: Value = serde_json::from_str(&resp)
                                .context("Couldn't parse URL response into JSON")?;
                            let id = value["id"].to_string().parse::<usize>()?;
                            return Ok(id);
                        }
                    }
                    Err(anyhow!("No english CSV record found with species ID: {id}",))
                }
                None => Err(anyhow!(
                    "No german or english version found for name: {name}"
                )),
            }
        } else {
            let value: Value = serde_json::from_str(&resp)?;
            let id = value["id"].to_string().parse::<usize>()?;
            Ok(id)
        }
    }

    pub async fn get_names(
        index: &Index,
        base_url: &str,
        language_url: &str,
    ) -> Result<Vec<String>> {
        let mut names = vec![];
        let url = format!("{}{}/", base_url, index.0);
        let resp = PokeApi::make_reqwest(&url).await?;
        if resp.to_lowercase() == "not found" {
            return Err(anyhow!("Nothing found for id: {index} at {url}",));
        } else {
            let value: Value = serde_json::from_str(&resp)?;
            let eng_name = value["species"]["name"].to_string().replace("\"", "");
            names.push(eng_name.to_lowercase());
        };

        // get german name
        let resp = PokeApi::make_reqwest(language_url).await?;

        // get csv data
        let mut pkm_species_id = None;
        let mut rdr = csv::Reader::from_reader(resp.as_bytes());
        for result in rdr.deserialize() {
            let record: csv_record::Record = result?;
            if names[0].to_lowercase() == record.name.to_lowercase() {
                pkm_species_id = Some(record.pokemon_species_id);
            }
        }
        match pkm_species_id {
            Some(id) => {
                let mut found = false;
                let mut rdr = csv::Reader::from_reader(resp.as_bytes());
                for result in rdr.deserialize() {
                    let record: csv_record::Record = result?;
                    if id == record.pokemon_species_id && record.local_language_id == 6 {
                        let ger_name = record.name;
                        names.push(ger_name.to_lowercase());
                        found = true;
                    }
                }
                if !found {
                    return Err(anyhow!("Couldn't find german name in csv file:URL: {language_url}\nspecies ID: {id}\nenglish name: {}", names[0],));
                }
            }
            None => {
                return Err(anyhow!(
                    "Couldn't find english name in csv file: URL: {language_url}\nName: `{}`",
                    names[0],
                ))
            }
        }
        Ok(names)
    }

    async fn make_reqwest(url: &str) -> Result<String> {
        reqwest::get(url)
            .await
            .with_context(|| format!("Couldn't reach URL: {}", url))?
            .text()
            .await
            .context("Couldn't fetch text from URL")
    }
}
