use crate::{Route, IS_AUTHENTICATED};
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    let nav = use_navigator();

    let handle_logout = move |_| {
        *IS_AUTHENTICATED.write() = false;
        nav.push(Route::Login);
    };

    rsx! {
        div { class: "nav-bar",
            Link { to: Route::Home,
                h1 { "Home" }
            }
            Link { to: Route::History, class: "search-history-link", "Search history" }
            button { class: "logout-button", onclick: handle_logout, "Logout" }
        }
        Outlet::<Route> {}
    }
}
