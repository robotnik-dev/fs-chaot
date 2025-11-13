use crate::{backend::validate_password, Route, IS_AUTHENTICATED};
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let nav = use_navigator();
    // when in dev mode, navigate to Home
    #[cfg(feature = "dev")]
    {
        use_effect(move || {
            debug!("Navigate to home");
            nav.push(Route::Home);
        });
    }
    let mut password = use_signal(String::new);
    let mut error = use_signal(String::new);

    // If already authenticated, redirect to home
    use_effect(move || {
        if *IS_AUTHENTICATED.read() {
            nav.push(Route::Home);
        }
    });

    let handle_submit = move |evt: FormEvent| async move {
        evt.prevent_default();

        let entered_password = password.read().clone();

        // Call server function to validate password
        match validate_password(entered_password).await {
            Ok(is_valid) => {
                if is_valid {
                    *IS_AUTHENTICATED.write() = true;
                    nav.push(Route::Home);
                } else {
                    error.set("Incorrect password".to_string());
                }
            }
            Err(e) => {
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
