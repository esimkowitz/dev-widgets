use dioxus::prelude::*;
use dioxus_router::prelude::*;
use strum_macros::EnumIter;

pub mod date_converter;
pub mod json_yaml_converter;
pub mod number_base_converter;

use crate::pages::{route_trait::WidgetRoute, Route, WidgetEntry};
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

fn Index(cx: Scope) -> Element {
    render! {
        div {
            class: "converter",
        }
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
}

impl Default for ConverterRoute {
    fn default() -> Self {
        Self::Index {}
    }
}
