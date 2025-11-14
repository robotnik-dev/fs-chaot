use crate::components::statistics::WidgetContainer;
use crate::statistics::{StatWidget, StatisticData};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn StatisticsDashboard(
    widgets_data: HashMap<String, (Box<dyn StatWidget>, Option<StatisticData>)>,
) -> Element {
    rsx! {
        div { class: "statistics-dashboard",
            div { class: "dashboard-grid",
                {

                    // Check if widget supports sorting

                    // Check if widget supports filtering

                    widgets_data
                        .iter()
                        .map(|(widget_id, (widget, data))| {
                            let metadata = widget.metadata();
                            let title = widget.title().to_string();
                            let description = metadata.description.clone();
                            let sort_orders = if let Some(sortable) = (widget.as_ref()
                                as &dyn std::any::Any)
                                .downcast_ref::<&dyn Sortable>()
                            {
                                sortable.available_sort_orders()
                            } else {
                                vec![]
                            };
                            let filter_criteria = if let Some(filterable) = (widget.as_ref()
                                as &dyn std::any::Any)
                                .downcast_ref::<&dyn Filterable>()
                            {
                                filterable.available_filters()
                            } else {
                                vec![]
                            };
                            rsx! {
                                WidgetContainer {
                                    key: "{widget_id}",
                                    widget_id: widget_id.clone(),
                                    title,
                                    description,
                                    data: data.clone(),
                                    metadata,
                                    sort_orders,
                                    filter_criteria,
                                }
                            }
                        })
                }
            }
        }
    }
}
