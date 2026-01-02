#![allow(non_snake_case)]
pub mod assets;
pub mod components;
pub mod environment;
pub mod pages;
pub mod utils;

use dioxus::prelude::*;

use crate::pages::Route;

pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: assets::CSS }
        document::Script { src: assets::BOOTSTRAP }
        document::Script { src: assets::GHPAGES }
        Router::<Route> {}
        document::Script { src: assets::DARKMODE }
    }
}
