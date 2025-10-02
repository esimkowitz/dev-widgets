use dioxus::prelude::*;
use dioxus_router::prelude::*;
use strum_macros::EnumIter;

pub mod hash_generator;
pub mod qr_code_generator;
pub mod uuid_generator;

use crate::pages::{Route, WidgetEntry, WidgetRoute};
use hash_generator::HashGenerator;
use qr_code_generator::QrCodeGenerator;
use uuid_generator::UuidGenerator;

#[derive(Clone, Debug, EnumIter, PartialEq, Routable)]
pub enum GeneratorRoute {
    #[route("/")]
    Index {},
    #[route("/hash")]
    HashGenerator {},
    #[route("/qr-code")]
    QrCodeGenerator {},
    #[route("/uuid")]
    UuidGenerator {},
}

fn Index() -> Element {
    rsx! {
        div { class: "generator" }
    }
}

impl WidgetRoute for GeneratorRoute {
    fn get_widget_routes() -> Vec<Route> {
        Self::get_widgets()
            .iter()
            .map(|widget| Route::Generator {
                child: widget.clone(),
            })
            .collect()
    }

    fn get_widget_type_string() -> &'static str {
        "Generator"
    }

    fn get_widget_entry(&self) -> Option<&'static WidgetEntry> {
        match self {
            Self::HashGenerator { .. } => Some(&hash_generator::WIDGET_ENTRY),
            Self::QrCodeGenerator { .. } => Some(&qr_code_generator::WIDGET_ENTRY),
            Self::UuidGenerator { .. } => Some(&uuid_generator::WIDGET_ENTRY),
            _ => None,
        }
    }
}

impl Default for GeneratorRoute {
    fn default() -> Self {
        Self::Index {}
    }
}
