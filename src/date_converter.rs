use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsHash;
use once_cell::sync::Lazy;

use crate::{widget_entry::WidgetEntry, sidebar_icon::SidebarIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Date Converter",
    short_title: "Date",
    description: "Convert dates between formats",
    path: "/date-converter",
    function: date_converter,
    icon: move |cx| SIDEBAR_ICON.sidebar_icon(cx),
};

const SIDEBAR_ICON: SidebarIcon<BsHash> = SidebarIcon {
    icon: BsHash,
};

pub fn date_converter(cx: Scope) -> Element {
    let x = Box::new(BsHash);
    cx.render(rsx! {
        div {
            class: "date-converter"
        }
    })
}
