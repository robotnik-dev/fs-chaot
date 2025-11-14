use dioxus::prelude::*;

#[component]
pub fn BookNavigation(
    current_page: Signal<usize>,
    total_pages: usize,
    on_search: EventHandler<String>,
    loading_card: Signal<bool>,
) -> Element {
    let mut search_input = use_signal(String::new);

    rsx! {
        div { class: "book-nav",
            if loading_card() {
                div { "Loading ..." }
            }
            input {
                r#type: "text",
                placeholder: "Search ID or name...",
                class: "book-nav__search",
                value: "{search_input}",
                oninput: move |e| search_input.set(e.value().clone()),
                onkeypress: move |e: Event<KeyboardData>| {
                    if e.key() == Key::Enter {
                        on_search.call(search_input().clone());
                    }
                },
            }
            button {
                class: "book-nav__button",
                disabled: current_page() == 1,
                onclick: move |_| {
                    if current_page() > 1 {
                        current_page.set(current_page() - 1);
                    }
                },
                "◀ Previous"
            }
            select {
                class: "book-nav__dropdown",
                value: "{current_page()}",
                onchange: move |e| {
                    if let Ok(page) = e.value().parse::<usize>() {
                        if page >= 1 && page <= total_pages {
                            current_page.set(page);
                        }
                    }
                },
                {(1..=total_pages).map(|p| rsx! {
                    option { value: "{p}", "Page {p}" }
                })}
            }
            button {
                class: "book-nav__button",
                disabled: current_page() >= total_pages,
                onclick: move |_| {
                    if current_page() < total_pages {
                        current_page.set(current_page() + 1);
                    }
                },
                "Next ▶"
            }
        }
    }
}
