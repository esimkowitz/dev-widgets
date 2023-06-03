#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*};

pub mod number_base_converter;
pub mod base64_converter;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        number_base_converter::NumberBaseConverter {}
        base64_converter::Base64Converter {}
    })
}