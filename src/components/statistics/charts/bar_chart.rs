use crate::statistics::{BarChartEntry, StatisticMetadata};
use dioxus::prelude::*;

#[component]
pub fn BarChart(mut entries: Vec<BarChartEntry>, metadata: StatisticMetadata) -> Element {
    entries.sort_by_key(|entry| entry.value as usize);
    entries.reverse();
    rsx! {
        div { class: "bar-chart",
            for entry in entries {
                div { class: "bar-chart__row", key: "{entry.label}",
                    span { class: "bar-chart__label", title: "{entry.label}",
                        {
                            if let Some(abbr) = entry.metadata.get("abbreviation") {
                                format!("{}({})", entry.label, abbr)
                            } else {
                                "{entry.label}".to_string()
                            }
                        }
                    }
                    div { class: "bar-chart__bar-container",
                        div {
                            class: "bar-chart__bar",
                            style: "width: {entry.value}%",
                        }
                        {
                            if let (Some(secret_cards), Some(secret_cards_percentage)) = (
                                entry.metadata.get("secret_cards"),
                                entry.metadata.get("secret_cards_percentage"),
                            ) {
                                let secret_cards = secret_cards.parse::<usize>().unwrap_or_default();
                                rsx! {
                                    div {
                                        class: "bar-chart__marker",
                                        style: if secret_cards == 0 { format!("left: 101%") } else { format!("left: {}%", secret_cards_percentage) },
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        }
                    }
                    span { class: "bar-chart__value",
                        {
                            if let (Some(owned), Some(total), Some(secret_cards)) = (
                                entry.metadata.get("owned"),
                                entry.metadata.get("total"),
                                entry.metadata.get("secret_cards"),
                            ) {
                                format!("{}/{}(s*{})({:.1}%)", owned, total, secret_cards, entry.value)
                            } else {
                                format!("{:.1}%", entry.value)
                            }
                        }
                    }
                }
            }
        }
    }
}
