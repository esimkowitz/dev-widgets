use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaShuffle;
use dioxus_free_icons::Icon;
use strum_macros::EnumIter;

pub mod date_converter;
pub mod json_yaml_converter;
pub mod number_base_converter;

use crate::pages::{CategoryEntry, Route, WidgetEntry, WidgetRoute};

pub static CATEGORY_ENTRY: CategoryEntry = CategoryEntry {
    title: "Converter",
    description: "Convert data between different formats",
    icon: || {
        rsx! {
            Icon::<FaShuffle> { class: "icon", icon: FaShuffle }
        }
    },
};
use date_converter::DateConverter;
use json_yaml_converter::JsonYamlConverter;
use number_base_converter::NumberBaseConverter;

#[derive(Clone, Debug, EnumIter, PartialEq, Routable)]
pub enum ConverterRoute {
    #[route("/")]
    Index {},
    #[route("/date")]
    DateConverter {},
    #[route("/json-yaml")]
    JsonYamlConverter {},
    #[route("/number-base")]
    NumberBaseConverter {},
}

fn Index() -> Element {
    rsx! {
        crate::pages::home_page::WidgetGrid { category_filter: Some("Converter") }
    }
}

impl WidgetRoute for ConverterRoute {
    fn get_widget_routes() -> Vec<Route> {
        Self::get_widgets()
            .iter()
            .map(|widget| Route::Converter {
                child: widget.clone(),
            })
            .collect()
    }

    fn get_widget_type_string() -> &'static str {
        "Converter"
    }

    fn get_widget_entry(&self) -> Option<&'static WidgetEntry> {
        match self {
            Self::DateConverter { .. } => Some(&date_converter::WIDGET_ENTRY),
            Self::JsonYamlConverter { .. } => Some(&json_yaml_converter::WIDGET_ENTRY),
            Self::NumberBaseConverter { .. } => Some(&number_base_converter::WIDGET_ENTRY),
            _ => None,
        }
    }

    fn get_category_entry() -> &'static CategoryEntry {
        &CATEGORY_ENTRY
    }
}

impl Default for ConverterRoute {
    fn default() -> Self {
        Self::Index {}
    }
}
