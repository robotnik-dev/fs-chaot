use crate::components::statistics::{BarChart, ScalarDisplay};
use crate::statistics::{ExpansionCompletionWidget, StatWidget, TotalOwnedWidget};
use dioxus::prelude::*;

#[component]
pub fn Statistics() -> Element {
    let mut expansion_data = use_signal(|| None);
    let mut total_owned_data = use_signal(|| None);
    let mut loading = use_signal(|| true);
    let mut error = use_signal(String::new);

    // Load all widget data on mount
    use_effect(move || {
        spawn(async move {
            // Calculate expansion completion
            let expansion_widget = ExpansionCompletionWidget;
            match expansion_widget.calculate().await {
                Ok(data) => expansion_data.set(Some(data)),
                Err(e) => {
                    #[cfg(feature = "server")]
                    tracing::error!(error = %e, "failed to calculate expansion completion");
                    error.set(format!("Failed to load expansion statistics: {}", e));
                }
            }

            // Calculate total owned
            let total_widget = TotalOwnedWidget;
            match total_widget.calculate().await {
                Ok(data) => total_owned_data.set(Some(data)),
                Err(e) => {
                    #[cfg(feature = "server")]
                    tracing::error!(error = %e, "failed to calculate total owned");
                    error.set(format!("Failed to load total owned: {}", e));
                }
            }

            loading.set(false);
        });
    });

    let expansion_widget = ExpansionCompletionWidget;
    let total_widget = TotalOwnedWidget;

    rsx! {
        div { class: "statistics-container",
            h1 { class: "statistics-title", "Collection Statistics" }

            if loading() {
                div { class: "statistics-loading", "Loading statistics..." }
            } else if !error.read().is_empty() {
                div { class: "statistics-error", "Error: {error}" }
            } else {
                div { class: "statistics-dashboard",
                    div { class: "dashboard-grid",
                        // Total Owned Widget
                        div { class: "widget-container",
                            div { class: "widget-header",
                                h3 { class: "widget-title", "{total_widget.title()}" }
                                p { class: "widget-description",
                                    "{total_widget.metadata().description}"
                                }
                            }
                            div { class: "widget-content",
                                {
                                    if let Some(ref data) = *total_owned_data.read() {
                                        match data {
                                            crate::statistics::StatisticData::Scalar(ref value) => rsx! {
                                                ScalarDisplay { value: value.clone() }
                                            },
                                            _ => rsx! {
                                                div { "Invalid data type" }
                                            },
                                        }
                                    } else {
                                        rsx! {
                                            div { class: "widget-loading", "Loading..." }
                                        }
                                    }
                                }
                            }
                        }

                        // Expansion Completion Widget
                        div { class: "widget-container",
                            div { class: "widget-header",
                                h3 { class: "widget-title", "{expansion_widget.title()}" }
                                p { class: "widget-description",
                                    "{expansion_widget.metadata().description}"
                                }
                            }
                            div { class: "widget-content",
                                {
                                    if let Some(ref data) = *expansion_data.read() {
                                        match data {
                                            crate::statistics::StatisticData::BarChart(ref entries) => {
                                                rsx! {
                                                    BarChart { entries: entries.clone(), metadata: expansion_widget.metadata() }
                                                }
                                            }
                                            _ => rsx! {
                                                div { "Invalid data type" }
                                            },
                                        }
                                    } else {
                                        rsx! {
                                            div { class: "widget-loading", "Loading..." }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
