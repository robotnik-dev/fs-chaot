use crate::backend::get_collection_statistics_db;
use crate::statistics::{
    AxisLabels, ChartType, ColorScheme, ScalarValue, StatWidget, StatisticData, StatisticMetadata,
};
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

pub struct TotalOwnedWidget;

impl StatWidget for TotalOwnedWidget {
    fn widget_id(&self) -> &'static str {
        "total_owned"
    }

    fn title(&self) -> &str {
        "Total Cards Owned"
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn calculate(&self) -> Pin<Box<dyn Future<Output = Result<StatisticData>> + Send + '_>> {
        Box::pin(async move {
            let stats = get_collection_statistics_db().await?;

            let value = ScalarValue {
                value: stats.total_cards_owned as f64,
                label: format!(
                    "{} / {}",
                    stats.total_cards_owned, stats.total_unique_pokemon
                ),
                unit: Some("cards".to_string()),
            };

            Ok(StatisticData::Scalar(value))
        })
    }

    #[cfg(target_arch = "wasm32")]
    fn calculate(&self) -> Pin<Box<dyn Future<Output = Result<StatisticData>> + '_>> {
        Box::pin(async move {
            let stats = get_collection_statistics_db().await?;

            let value = ScalarValue {
                value: stats.total_cards_owned as f64,
                label: format!(
                    "{} / {}",
                    stats.total_cards_owned, stats.total_unique_pokemon
                ),
                unit: Some("cards".to_string()),
            };

            Ok(StatisticData::Scalar(value))
        })
    }

    fn metadata(&self) -> StatisticMetadata {
        StatisticMetadata {
            chart_type: ChartType::Scalar,
            color_scheme: ColorScheme::Default,
            axis_labels: AxisLabels {
                x_label: None,
                y_label: None,
            },
            description: "Total unique Pokemon cards in your collection".to_string(),
        }
    }
}
