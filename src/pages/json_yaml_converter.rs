#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsFileText;

use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "JSON <> YAML Converter",
    short_title: "JSON <> YAML",
    description: "Convert between JSON and YAML file formats",
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsFileText> = WidgetIcon { icon: BsFileText };

pub fn JsonYamlConverter(cx: Scope) -> Element {
    render! {
        div {
            class: "json-yaml-converter"
        }
    }
}
