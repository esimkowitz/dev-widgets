use dioxus::prelude::*;

use crate::widget_entry;

pub const WIDGET_ENTRY: widget_entry::WidgetEntry = widget_entry::WidgetEntry {
    title: "Color Picker",
    description: "Pick a color and get its output in different formats",
    widget_type: widget_entry::WidgetType::Media,
    widget: widget_entry::Widget::ColorPicker,
    function: color_picker,
};

pub fn color_picker(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div {
                class: "widget-title",
                WIDGET_ENTRY.title
            }
        }
    })
}
