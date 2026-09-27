#![allow(non_snake_case)]
pub mod assets;
pub mod components;
pub mod environment;
pub mod pages;
pub mod utils;

use dioxus::prelude::*;

use crate::pages::Route;

pub fn App() -> Element {
    // Don't mount the app until the stylesheet has loaded. A stylesheet inserted at runtime
    // doesn't block rendering, so the first frames would paint unstyled and the mobile nav
    // drawer would visibly animate from that state to hidden.
    let mut css_ready = use_signal(|| false);

    rsx! {
        link {
            rel: "stylesheet",
            href: assets::CSS,
            onload: move |_| css_ready.set(true),
            // Render unstyled rather than not at all if the stylesheet fails
            onerror: move |_| css_ready.set(true),
        }
        document::Script { src: assets::GHPAGES }
        if css_ready() {
            Router::<Route> {}
        }
        document::Script { src: assets::DARKMODE }
    }
}
