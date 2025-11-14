use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "nav-bar",
            Link { to: Route::Collection,
                h1 { "My Collection" }
            }
            Link { to: Route::Statistics, class: "nav-bar__link", "Statistics" }
        }
        Outlet::<Route> {}
    }
}
