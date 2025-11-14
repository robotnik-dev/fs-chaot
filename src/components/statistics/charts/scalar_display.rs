use crate::statistics::ScalarValue;
use dioxus::prelude::*;

#[component]
pub fn ScalarDisplay(value: ScalarValue) -> Element {
    rsx! {
        div { class: "scalar-display",
            div { class: "scalar-display__value", "{value.label}" }
            if let Some(unit) = value.unit {
                div { class: "scalar-display__unit", "{unit}" }
            }
        }
    }
}
