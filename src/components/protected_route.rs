use super::NavBar;
use crate::{Route, IS_AUTHENTICATED};
use dioxus::prelude::*;

#[component]
pub fn ProtectedRoute() -> Element {
    let nav = use_navigator();

    // Check authentication on mount and whenever it changes
    use_effect(move || {
        if !*IS_AUTHENTICATED.read() {
            nav.push(Route::Login);
        }
    });

    // Only render children if authenticated
    if *IS_AUTHENTICATED.read() {
        rsx! {
            NavBar {}
        }
    } else {
        rsx! {
            div { "Redirecting to login..." }
        }
    }
}
