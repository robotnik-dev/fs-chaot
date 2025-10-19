use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "title",
            h1 { "Chaot" }
        }
        div { class: "input-group",
            input {
                r#type: "text",
                autofocus: true,
                name: "text",
                class: "input",
                placeholder: "Name or ID",
            }
        }
    }
}
