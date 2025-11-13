use super::NavBar;
use crate::IS_AUTHENTICATED;
use dioxus::prelude::*;

#[component]
pub fn ProtectedRoute() -> Element {
    #[cfg(feature = "dev")]
    {
        use_effect(move || {
            *IS_AUTHENTICATED.write() = true;
        });
    }

    // Check authentication on mount and whenever it changes
    #[cfg(not(feature = "dev"))]
    {
        use crate::Route;
        let nav = use_navigator();
        use_effect(move || {
            if !*IS_AUTHENTICATED.read() {
                nav.push(Route::Login);
            }
        });
    }

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
