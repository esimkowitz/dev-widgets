#![allow(non_snake_case)]
pub mod components;
pub mod environment;
pub mod pages;
pub mod utils;
pub mod assets;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::Route;

pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: assets::CSS }
        document::Script { src: assets::bootstrap }
        document::Script { src: assets::ghpages }
        Router::<Route> {}
        document::Script { src: assets::darkmode }
    }
}
