#![allow(non_snake_case)]
pub mod components;
pub mod environment;
pub mod pages;
pub mod persistence;
pub mod utils;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::Route;

pub fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
