use dioxus::prelude::*;

use crate::widget_entry;

pub const WIDGET_ENTRY: widget_entry::WidgetEntry = widget_entry::WidgetEntry {
    title: "Date Converter",
    description: "Convert dates between formats",
    path: "/date-converter",
    function: date_converter,
};

pub fn date_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "date-converter"
        }
    })
}
