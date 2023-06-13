use dioxus::prelude::*;

use crate::widget_entry;

pub const WIDGET_ENTRY: widget_entry::WidgetEntry = widget_entry::WidgetEntry {
    title: "JSON <> YAML Converter",
    description: "Convert between JSON and YAML file formats",
    path: "/json-yaml-converter",
    function: json_yaml_converter,
};

pub fn json_yaml_converter<'a>(cx: &'a ScopeState) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "json-yaml-converter"
        }
    })
}
