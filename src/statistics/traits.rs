use crate::statistics::{FilterCriteria, SortOrder, StatisticData, StatisticMetadata};
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

/// Core trait for all statistics widgets
///
/// Each widget is responsible for:
/// - Fetching its data
/// - Calculating statistics
/// - Providing metadata for rendering
///
/// Widgets are immutable after calculation for predictable state.
pub trait StatWidget: Send + Sync {
    /// Unique identifier for the widget (e.g., "expansion_completion")
    #[allow(dead_code)]
    fn widget_id(&self) -> &'static str;

    /// Human-readable title for dashboard display
    fn title(&self) -> &str;

    /// Fetch and calculate statistics data
    /// Called once during dashboard initialization
    #[cfg(not(target_arch = "wasm32"))]
    fn calculate(&self) -> Pin<Box<dyn Future<Output = Result<StatisticData>> + Send + '_>>;

    /// Fetch and calculate statistics data (wasm32 version without Send bound)
    /// Called once during dashboard initialization
    #[cfg(target_arch = "wasm32")]
    fn calculate(&self) -> Pin<Box<dyn Future<Output = Result<StatisticData>> + '_>>;

    /// Metadata for rendering (chart type, colors, labels)
    fn metadata(&self) -> StatisticMetadata;
}

/// Trait for widgets that support filtering
#[allow(dead_code)]
pub trait Filterable: StatWidget {
    /// Apply filter criteria to the dataset
    /// Returns a new filtered StatisticData
    fn apply_filter(
        &self,
        data: &StatisticData,
        criteria: &FilterCriteria,
    ) -> Result<StatisticData>;

    /// Available filter options for this widget
    fn available_filters(&self) -> Vec<FilterCriteria>;
}

/// Trait for widgets that support sorting
#[allow(dead_code)]
pub trait Sortable: StatWidget {
    /// Sort data by specified order
    fn apply_sort(&self, data: &StatisticData, order: &SortOrder) -> StatisticData;

    /// Available sort options
    fn available_sort_orders(&self) -> Vec<SortOrder>;
}

/// Trait for widgets that can be exported (CSV, JSON)
#[allow(dead_code)]
pub trait Exportable: StatWidget {
    /// Export to CSV format
    fn to_csv(&self, data: &StatisticData) -> Result<String>;

    /// Export to JSON format
    fn to_json(&self, data: &StatisticData) -> Result<String>;
}
