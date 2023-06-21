use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsHash;

use crate::{widget_entry::WidgetEntry, sidebar_icon::sidebar_icon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Date Converter",
    short_title: "Date",
    description: "Convert dates between formats",
    path: "/date-converter",
    function: date_converter,
    icon: sidebar_icon::<BsHash>,
};

pub fn date_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "date-converter"
        }
    })
}
