use crate::components::statistics::ChartRenderer;
use crate::statistics::{FilterCriteria, SortOrder, StatisticData};
use dioxus::prelude::*;

#[component]
pub fn WidgetContainer(
    widget_id: String,
    title: String,
    description: String,
    data: Option<StatisticData>,
    metadata: crate::statistics::StatisticMetadata,
    #[props(default = vec![])] sort_orders: Vec<SortOrder>,
    #[props(default = vec![])] filter_criteria: Vec<FilterCriteria>,
) -> Element {
    let mut current_data = use_signal(|| data.clone());
    let mut active_sort = use_signal(|| None::<SortOrder>);
    let mut active_filter = use_signal(|| None::<FilterCriteria>);

    // Update current_data when data prop changes
    use_effect(move || {
        if let Some(d) = data.clone() {
            current_data.set(Some(d));
        }
    });

    rsx! {
        div { class: "widget-container", id: "widget-{widget_id}",
            div { class: "widget-header",
                h3 { class: "widget-title", "{title}" }
                p { class: "widget-description", "{description}" }
            }

            // Controls (if sorting or filtering is available)
            if !sort_orders.is_empty() || !filter_criteria.is_empty() {
                div { class: "widget-controls",
                    if !sort_orders.is_empty() {
                        div { class: "widget-controls__sort",
                            label { "Sort by: " }
                            select {
                                class: "widget-controls__select",
                                onchange: move |evt| {
                                    let value = evt.value();
                                    let sort_order = match value.as_str() {
                                        "completion_desc" => Some(SortOrder::CompletionDesc),
                                        "completion_asc" => Some(SortOrder::CompletionAsc),
                                        "alphabetical" => Some(SortOrder::Alphabetical),
                                        _ => None,
                                    };
                                    active_sort.set(sort_order);
                                    // Note: Actual sorting would need to be implemented via callback
                                },
                                option { value: "", "Default" }
                                if sort_orders.contains(&SortOrder::CompletionDesc) {
                                    option { value: "completion_desc", "Completion % (High to Low)" }
                                }
                                if sort_orders.contains(&SortOrder::CompletionAsc) {
                                    option { value: "completion_asc", "Completion % (Low to High)" }
                                }
                                if sort_orders.contains(&SortOrder::Alphabetical) {
                                    option { value: "alphabetical", "Alphabetical" }
                                }
                            }
                        }
                    }

                    if !filter_criteria.is_empty() {
                        div { class: "widget-controls__filter",
                            label { "Filter: " }
                            select {
                                class: "widget-controls__select",
                                onchange: move |evt| {
                                    let value = evt.value();
                                    let filter = match value.as_str() {
                                        "min_25" => Some(FilterCriteria::MinCompletion(25.0)),
                                        "min_50" => Some(FilterCriteria::MinCompletion(50.0)),
                                        "min_75" => Some(FilterCriteria::MinCompletion(75.0)),
                                        _ => None,
                                    };
                                    active_filter.set(filter);
                                    // Note: Actual filtering would need to be implemented via callback
                                },
                                option { value: "", "All Expansions" }
                                if filter_criteria.iter().any(|f| matches!(f, FilterCriteria::MinCompletion(25.0))) {
                                    option { value: "min_25", "≥ 25% Complete" }
                                }
                                if filter_criteria.iter().any(|f| matches!(f, FilterCriteria::MinCompletion(50.0))) {
                                    option { value: "min_50", "≥ 50% Complete" }
                                }
                                if filter_criteria.iter().any(|f| matches!(f, FilterCriteria::MinCompletion(75.0))) {
                                    option { value: "min_75", "≥ 75% Complete" }
                                }
                            }
                        }
                    }
                }
            }

            // Widget content
            div { class: "widget-content",
                match current_data() {
                    Some(d) => rsx! {
                        ChartRenderer { data: d, metadata: metadata.clone() }
                    },
                    None => rsx! {
                        div { class: "widget-loading", "Loading..." }
                    },
                }
            }
        }
    }
}
