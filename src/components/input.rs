use dioxus::prelude::*;

#[component]
pub fn Input(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        input { class: "input", ..attributes, {children} }
    }
}
