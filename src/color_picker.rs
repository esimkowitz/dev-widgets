use dioxus::prelude::*;

use crate::widget_entry;

pub const WIDGET_ENTRY: widget_entry::WidgetEntry = widget_entry::WidgetEntry {
    title: "Color Picker",
    description: "Pick a color and get its output in different formats",
    path: "/color-picker",
    function: color_picker,
};

pub fn color_picker(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "color-picker"
        }
    })
}
