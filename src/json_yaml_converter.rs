use dioxus::prelude::*;

use crate::widget_entry;

pub const WIDGET_ENTRY: widget_entry::WidgetEntry = widget_entry::WidgetEntry {
    title: "JSON <> YAML Converter",
    short_title: "JSON <> YAML",
    description: "Convert between JSON and YAML file formats",
    path: "/json-yaml-converter",
    function: json_yaml_converter,
};

pub fn json_yaml_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "json-yaml-converter"
        }
    })
}
