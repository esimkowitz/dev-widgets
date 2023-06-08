use dioxus::prelude::*;

use crate::widget_entry;

pub const WIDGET_ENTRY: widget_entry::WidgetEntry = widget_entry::WidgetEntry {
    title: "JSON <> YAML Converter",
    description: "Convert between JSON and YAML file formats",
    widget: widget_entry::Widget::JsonYamlConverter,
    function: json_yaml_converter,
};

pub fn json_yaml_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div {
                class: "widget-title",
                WIDGET_ENTRY.title
            }
        }
    })
}
