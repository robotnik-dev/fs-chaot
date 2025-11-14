use crate::components::statistics::{BarChart, ScalarDisplay};
use crate::statistics::{StatisticData, StatisticMetadata};
use dioxus::prelude::*;

#[component]
pub fn ChartRenderer(data: StatisticData, metadata: StatisticMetadata) -> Element {
    match data {
        StatisticData::BarChart(entries) => rsx! {
            BarChart { entries, metadata }
        },
        StatisticData::Scalar(value) => rsx! {
            ScalarDisplay { value }
        },
        StatisticData::PieChart(_segments) => rsx! {
            div { class: "chart-not-implemented", "Pie chart visualization not yet implemented" }
        },
        StatisticData::TimeSeries(_points) => rsx! {
            div { class: "chart-not-implemented", "Time series visualization not yet implemented" }
        },
    }
}
