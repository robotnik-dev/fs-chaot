use crate::card::Card;
use crate::components::*;
use dioxus::prelude::*;

mod backend;
mod card;
mod components;
mod csv_record;
mod pokeapi;

pub const BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
pub const LANGUAGE_URL: &str = "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv/pokemon_species_names.csv";
pub const SPRITE_URL: &str =
    "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/";
pub const CARDS_PER_BOOK: usize = 576;
pub const CARDS_PER_PAGE: usize = 24;
static STYLE: Asset = asset!("/assets/style.css");
static THEME: Asset = asset!("/assets/dx-components-theme.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    SearchView,

    #[route("/history")]
    History,
}

fn main() {
    dioxus::launch(App);
}

static CARDS: GlobalSignal<Vec<(usize, Card)>> = Signal::global(Vec::new);

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: STYLE }
        document::Stylesheet { href: THEME }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
