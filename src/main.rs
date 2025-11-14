use crate::{card::Card, components::*};
use dioxus::prelude::*;

mod backend;
mod card;
mod components;
mod csv_record;
mod expansion;
mod logging;
mod pokeapi;
mod statistics;
mod utils;

pub const BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
pub const LANGUAGE_URL: &str = "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv/pokemon_species_names.csv";
pub const SPRITE_URL: &str =
    "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/";
pub const CARDS_PER_BOOK: usize = 576;
pub const BOOKS: usize = 2;
pub const CARDS_PER_DOUBLE_PAGE: usize = 24;
pub const MAX_POKEMON: usize = 1025;
pub const TOTAL_PAGES: usize = MAX_POKEMON.div_ceil(CARDS_PER_DOUBLE_PAGE);

const FAVICON: Asset = asset!("/assets/favicon.ico");
static STYLE: Asset = asset!("/assets/style.css");
static THEME: Asset = asset!("/assets/dx-components-theme.css");

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Login,

    #[layout(ProtectedRoute)]
    #[route("/collection")]
    Collection,

    #[route("/statistics")]
    Statistics,
}

fn main() {
    // Initialize logging
    #[cfg(feature = "server")]
    init_server_logging();

    #[cfg(not(feature = "server"))]
    dioxus::fullstack::set_server_url("https://fs-chaot-production.up.railway.app");

    dioxus::launch(App);
}

/// Initialize server-side logging with tracing-subscriber
#[cfg(feature = "server")]
fn init_server_logging() {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = if cfg!(feature = "dev") {
        // Dev: Show debug and above, exclude noisy crates
        EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new("debug,hyper=info,tower=info,tokio=info,dioxus_core=info")
        })
    } else {
        // Prod: Show info and above by default
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(cfg!(feature = "dev"))
        .init();

    tracing::info!("Logging initialized");
}

pub static CARDS: GlobalSignal<Vec<(usize, Card)>> = Signal::global(Vec::new);
pub static IS_AUTHENTICATED: GlobalSignal<bool> = Signal::global(|| false);

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: STYLE }
        document::Stylesheet { href: THEME }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
