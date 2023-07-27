#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsEyedropper;

use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Color Picker",
    short_title: "Color Picker",
    description: "Pick a color and get its output in different formats",
    path: "/color-picker",
    function: ColorPicker,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsEyedropper> = WidgetIcon { icon: BsEyedropper };

pub fn ColorPicker(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "color-picker",
            div {
                class: "colorwheel-wrapper",
                div {
                    class: "colorwheel-gradient",
                    ColorWheelOverlay {}
                }
            }
        }
    })
}

fn ColorWheelOverlay(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "colorwheel-overlay",
            div {
                class: "colorwheel-inner"
            }
            div {
                class: "colorwheel-cursor"
            }
        }
    })
}