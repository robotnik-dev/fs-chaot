use crate::backend::get_expansion_statistics_db;
use crate::statistics::{
    AxisLabels, BarChartEntry, ChartType, ColorScheme, FilterCriteria, Filterable, SortOrder,
    Sortable, StatWidget, StatisticData, StatisticMetadata,
};
use anyhow::Result;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub struct ExpansionCompletionWidget;

impl StatWidget for ExpansionCompletionWidget {
    fn widget_id(&self) -> &'static str {
        "expansion_completion"
    }

    fn title(&self) -> &str {
        "Expansion Completion"
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn calculate(&self) -> Pin<Box<dyn Future<Output = Result<StatisticData>> + Send + '_>> {
        Box::pin(async move {
            let stats = get_expansion_statistics_db().await?;

            let entries = stats
                .into_iter()
                .map(|stat| {
                    let mut metadata = HashMap::new();
                    metadata.insert("expansion_id".to_string(), stat.expansion.id.to_string());
                    metadata.insert("owned".to_string(), stat.owned_count.to_string());
                    metadata.insert("total".to_string(), stat.total_count.to_string());
                    metadata.insert(
                        "abbreviation".to_string(),
                        stat.expansion.abbreviation.clone(),
                    );

                    BarChartEntry {
                        label: stat.expansion.name.clone(),
                        value: stat.completion_percentage(),
                        metadata,
                    }
                })
                .collect();

            Ok(StatisticData::BarChart(entries))
        })
    }

    #[cfg(target_arch = "wasm32")]
    fn calculate(&self) -> Pin<Box<dyn Future<Output = Result<StatisticData>> + '_>> {
        Box::pin(async move {
            let stats = get_expansion_statistics_db().await?;

            let entries = stats
                .into_iter()
                .map(|stat| {
                    let mut metadata = HashMap::new();
                    metadata.insert("expansion_id".to_string(), stat.expansion.id.to_string());
                    metadata.insert("owned".to_string(), stat.owned_count.to_string());
                    metadata.insert("total".to_string(), stat.total_count.to_string());
                    metadata.insert(
                        "abbreviation".to_string(),
                        stat.expansion.abbreviation.clone(),
                    );

                    BarChartEntry {
                        label: stat.expansion.name.clone(),
                        value: stat.completion_percentage(),
                        metadata,
                    }
                })
                .collect();

            Ok(StatisticData::BarChart(entries))
        })
    }

    fn metadata(&self) -> StatisticMetadata {
        StatisticMetadata {
            chart_type: ChartType::HorizontalBar,
            color_scheme: ColorScheme::CompletionGradient,
            axis_labels: AxisLabels {
                x_label: Some("Completion Percentage".to_string()),
                y_label: Some("Expansion".to_string()),
            },
            description: "Track your collection progress across all Pokemon TCG expansions"
                .to_string(),
        }
    }
}

impl Sortable for ExpansionCompletionWidget {
    fn apply_sort(&self, data: &StatisticData, order: &SortOrder) -> StatisticData {
        if let StatisticData::BarChart(entries) = data {
            let mut sorted = entries.clone();

            match order {
                SortOrder::CompletionDesc => {
                    sorted.sort_by(|a, b| b.value.total_cmp(&a.value));
                }
                SortOrder::CompletionAsc => {
                    sorted.sort_by(|a, b| a.value.total_cmp(&b.value));
                }
                SortOrder::Alphabetical => {
                    sorted.sort_by(|a, b| a.label.cmp(&b.label));
                }
                _ => {}
            }

            StatisticData::BarChart(sorted)
        } else {
            data.clone()
        }
    }

    fn available_sort_orders(&self) -> Vec<SortOrder> {
        vec![
            SortOrder::CompletionDesc,
            SortOrder::CompletionAsc,
            SortOrder::Alphabetical,
        ]
    }
}

impl Filterable for ExpansionCompletionWidget {
    fn apply_filter(
        &self,
        data: &StatisticData,
        criteria: &FilterCriteria,
    ) -> Result<StatisticData> {
        if let StatisticData::BarChart(entries) = data {
            let filtered: Vec<BarChartEntry> = match criteria {
                FilterCriteria::MinCompletion(min_percent) => entries
                    .iter()
                    .filter(|entry| entry.value >= *min_percent)
                    .cloned()
                    .collect(),
                FilterCriteria::ExpansionSeries(_series) => {
                    // TODO: Implement series filtering when expansion series data is available
                    entries.clone()
                }
                _ => entries.clone(),
            };

            Ok(StatisticData::BarChart(filtered))
        } else {
            Ok(data.clone())
        }
    }

    fn available_filters(&self) -> Vec<FilterCriteria> {
        vec![
            FilterCriteria::MinCompletion(0.0),
            FilterCriteria::MinCompletion(25.0),
            FilterCriteria::MinCompletion(50.0),
            FilterCriteria::MinCompletion(75.0),
        ]
    }
}
