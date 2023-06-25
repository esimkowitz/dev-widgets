use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsEyedropper;

use crate::{widget_entry::WidgetEntry, sidebar_icon::SidebarIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Color Picker",
    short_title: "Color Picker",
    description: "Pick a color and get its output in different formats",
    path: "/color-picker",
    function: color_picker,
    icon: move |cx| SIDEBAR_ICON.sidebar_icon(cx),
};

const SIDEBAR_ICON: SidebarIcon<BsEyedropper> = SidebarIcon {
    icon: BsEyedropper,
};

pub fn color_picker(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "color-picker"
        }
    })
}
