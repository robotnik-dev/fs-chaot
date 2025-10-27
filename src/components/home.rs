use dioxus::prelude::*;

use crate::components::{CardContainer, SearchBar};

#[component]
pub fn Home() -> Element {
    rsx! {
        SearchBar {}
        CardContainer {}
    }
}
