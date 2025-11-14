use crate::statistics::{BarChartEntry, StatisticMetadata};
use dioxus::prelude::*;

#[component]
pub fn BarChart(mut entries: Vec<BarChartEntry>, metadata: StatisticMetadata) -> Element {
    entries.sort_by_key(|entry| {
        entry
            .metadata
            .get("owned")
            .unwrap_or(&"0".to_string())
            .clone()
    });
    entries.reverse();
    rsx! {
        div { class: "bar-chart",
            for entry in entries {
                div { class: "bar-chart__row", key: "{entry.label}",
                    span { class: "bar-chart__label", title: "{entry.label}", "{entry.label}" }
                    div { class: "bar-chart__bar-container",
                        div {
                            class: "bar-chart__bar",
                            style: "width: {entry.value}%",
                        }
                    }
                    span { class: "bar-chart__value",
                        {
                            if let (Some(owned), Some(total)) = (
                                entry.metadata.get("owned"),
                                entry.metadata.get("total"),
                            ) {
                                format!("{}/{} ({:.1}%)", owned, total, entry.value)
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
