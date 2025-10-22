use dioxus::prelude::*;

use crate::components::{CardContainer, SearchBar};

#[component]
pub fn SearchView() -> Element {
    rsx! {
        SearchBar {}
        CardContainer {}
    }
}
