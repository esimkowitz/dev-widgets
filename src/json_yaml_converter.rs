use dioxus::prelude::*;

pub const TITLE: &str = "JSON <> YAML Converter";
pub const DESCRIPTION: &str = "Convert between JSON and YAML file formats";

pub fn json_yaml_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div {
                class: "widget-title",
                TITLE
            }
        }
    })
}
