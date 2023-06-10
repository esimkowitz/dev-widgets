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
            div {
                class: "widget-title",
                WIDGET_ENTRY.title
            }
            div {
                class: "widget-body d-flex flex-row",
                div {
                    class: "colorwheel-wrapper",
                    div {
                        class: "colorwheel-gradient",
                        div {
                            class: "colorwheel-inner"
                        }
                    }
                }
            }
        }
    })
}
