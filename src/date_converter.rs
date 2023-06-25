use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsClock;

use crate::widget_entry::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Date Converter",
    short_title: "Date",
    description: "Convert dates between formats",
    path: "/date-converter",
    function: date_converter,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsClock> = WidgetIcon {
    icon: BsClock,
};

pub fn date_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "date-converter"
        }
    })
}
