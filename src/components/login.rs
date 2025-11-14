use crate::{backend::validate_password, Route, IS_AUTHENTICATED};
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let nav = use_navigator();
    // when in dev mode, navigate to Collection
    #[cfg(feature = "dev")]
    {
        use_effect(move || {
            debug!("Navigate to collection view");
            nav.push(Route::Collection);
        });
    }
    let mut password = use_signal(String::new);
    let mut error = use_signal(String::new);

    // If already authenticated, redirect to home
    use_effect(move || {
        if *IS_AUTHENTICATED.read() {
            nav.push(Route::Collection);
        }
    });

    let handle_submit = move |evt: FormEvent| async move {
        evt.prevent_default();

        let entered_password = password.read().clone();

        tracing::debug!("user attempting authentication");

        // Call server function to validate password
        match validate_password(entered_password).await {
            Ok(is_valid) => {
                if is_valid {
                    tracing::info!("user authenticated successfully");
                    *IS_AUTHENTICATED.write() = true;
                    nav.push(Route::Collection);
                } else {
                    tracing::warn!("authentication failed - incorrect credentials");
                    error.set("Incorrect password".to_string());
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "authentication error");
                error.set(format!("Error: {}", e));
            }
        }
    };

    rsx! {
        div { class: "login-container",
            div { class: "login-box",
                h1 { "Login" }
                form { onsubmit: handle_submit,
                    div { class: "form-group",
                        label { r#for: "password", "Password:" }
                        input {
                            r#type: "password",
                            id: "password",
                            value: "{password}",
                            oninput: move |evt| password.set(evt.value()),
                            placeholder: "Enter password",
                            autofocus: true,
                        }
                    }
                    if !error.read().is_empty() {
                        div { class: "error", "{error}" }
                    }
                    button { r#type: "submit", "Login" }
                }
            }
        }
    }
}
