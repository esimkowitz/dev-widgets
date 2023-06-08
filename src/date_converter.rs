use dioxus::prelude::*;

use crate::widget_entry;

pub const WIDGET_ENTRY: widget_entry::WidgetEntry = widget_entry::WidgetEntry {
    title: "Date Converter",
    description: "Convert dates between formats",
    widget_type: widget_entry::WidgetType::Converter,
    widget: widget_entry::Widget::DateConverter,
    function: date_converter,
};

pub fn date_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div {
                class: "widget-title",
                WIDGET_ENTRY.title
            }
        }
    })
}
