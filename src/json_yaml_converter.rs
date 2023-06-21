use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsHash;

use crate::{widget_entry::WidgetEntry, sidebar_icon::sidebar_icon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "JSON <> YAML Converter",
    short_title: "JSON <> YAML",
    description: "Convert between JSON and YAML file formats",
    path: "/json-yaml-converter",
    function: json_yaml_converter,
    icon: sidebar_icon::<BsHash>,
};

pub fn json_yaml_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "json-yaml-converter"
        }
    })
}
