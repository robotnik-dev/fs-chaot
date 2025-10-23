use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "nav-bar",
            Link { to: Route::SearchView,
                h1 { "Home" }
            }
            Link { to: Route::History, class: "search-history-link", "Search history" }
        }
        Outlet::<Route> {}
    }
}
