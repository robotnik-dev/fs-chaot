use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::expansion::Expansion;

/// Represents calculated statistic data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StatisticData {
    /// Bar chart data: (label, value) pairs
    BarChart(Vec<BarChartEntry>),

    /// Single scalar value (e.g., total cards owned)
    Scalar(ScalarValue),

    /// Pie chart segments
    PieChart(Vec<PieChartSegment>),

    /// Time series data
    TimeSeries(Vec<TimeSeriesPoint>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BarChartEntry {
    pub label: String,
    pub value: f64,
    pub metadata: HashMap<String, String>, // e.g., {"expansion_id": "1", "total": "165"}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScalarValue {
    pub value: f64,
    pub label: String,
    pub unit: Option<String>, // e.g., "cards", "%"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PieChartSegment {
    pub label: String,
    pub value: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeSeriesPoint {
    pub timestamp: i64,
    pub value: f64,
}

/// Metadata for rendering widgets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatisticMetadata {
    pub chart_type: ChartType,
    pub color_scheme: ColorScheme,
    pub axis_labels: AxisLabels,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChartType {
    HorizontalBar,
    VerticalBar,
    Pie,
    Line,
    Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorScheme {
    Default,
    RarityBased,
    CompletionGradient,
    Custom(Vec<String>), // Hex color codes
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AxisLabels {
    pub x_label: Option<String>,
    pub y_label: Option<String>,
}

/// Filter criteria for statistics
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FilterCriteria {
    /// Filter by expansion series (e.g., "Sword & Shield")
    ExpansionSeries(String),

    /// Filter by minimum completion percentage
    MinCompletion(f64),

    /// Filter by date range
    DateRange { start: i64, end: i64 },

    /// Custom filter (extensible)
    Custom { key: String, value: String },
}

/// Sort order options
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    /// Sort by completion percentage (ascending)
    CompletionAsc,

    /// Sort by completion percentage (descending)
    CompletionDesc,

    /// Sort alphabetically by name
    Alphabetical,

    /// Sort by value (descending)
    ValueDesc,

    /// Sort by value (ascending)
    ValueAsc,
}

// ==================== Domain Models ====================

/// Expansion statistics with owned/total card counts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpansionStats {
    pub expansion: Expansion,
    pub owned_count: usize,
    pub total_count: usize,
    pub completion_rate: f64, // 0.0 to 1.0
}

impl ExpansionStats {
    pub fn new(expansion: Expansion, owned_count: usize) -> Self {
        let total_count = expansion.cards + expansion.secret_cards;
        let completion_rate = if total_count > 0 {
            owned_count as f64 / total_count as f64
        } else {
            0.0
        };

        Self {
            expansion,
            owned_count,
            total_count,
            completion_rate,
        }
    }

    /// Completion rate as percentage (0-100)
    pub fn completion_percentage(&self) -> f64 {
        self.completion_rate * 100.0
    }
}

/// Overall collection statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionStats {
    pub total_cards_owned: usize,
    pub total_unique_pokemon: usize, // 1025
    pub total_expansion_cards: usize,
    pub overall_completion_rate: f64,
}
