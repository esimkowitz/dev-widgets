#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaFileLines;

use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "JSON <> YAML Converter",
    short_title: "JSON <> YAML",
    description: "Convert between JSON and YAML file formats",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<FaFileLines> = WidgetIcon { icon: FaFileLines };

pub fn JsonYamlConverter() -> Element {
    rsx! {
        div { class: "json-yaml-converter",
            div { class: "alert alert-warning", "JSON <> YAML converter is not implemented yet." }
        }
    }
}
