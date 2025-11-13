use crate::{backend::get_card_expansions_db, card::Rarity};

pub async fn get_highest_rarity(index: usize) -> Rarity {
    if let Ok(mut expansions) = get_card_expansions_db(index).await {
        expansions.sort_by_key(|e| e.clone().rarity);
        expansions
            .iter()
            .map(|e| e.clone().rarity)
            .next_back()
            .unwrap_or_default()
    } else {
        Rarity::Common
    }
}
