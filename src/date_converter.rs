use dioxus::prelude::*;

pub const TITLE: &str = "Date Converter";
pub const DESCRIPTION: &str = "Convert dates between formats";

pub fn date_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div {
                class: "widget-title",
                TITLE
            }
        }
    })
}
