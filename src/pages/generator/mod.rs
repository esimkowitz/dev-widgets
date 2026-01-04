use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaScrewdriverWrench;
use dioxus_free_icons::Icon;
use strum_macros::EnumIter;

pub mod hash_generator;
pub mod lorem_ipsum;
pub mod qr_code_generator;
pub mod uuid_generator;

use crate::pages::{CategoryEntry, Route, WidgetEntry, WidgetRoute};

pub static CATEGORY_ENTRY: CategoryEntry = CategoryEntry {
    title: "Generator",
    description: "Generate hashes, codes, and identifiers",
    icon: || {
        rsx! {
            Icon::<FaScrewdriverWrench> { class: "icon", icon: FaScrewdriverWrench }
        }
    },
};
use hash_generator::HashGenerator;
use lorem_ipsum::LoremIpsum;
use qr_code_generator::QrCodeGenerator;
use uuid_generator::UuidGenerator;

#[derive(Clone, Debug, EnumIter, PartialEq, Routable)]
pub enum GeneratorRoute {
    #[route("/")]
    Index {},
    #[route("/hash")]
    HashGenerator {},
    #[route("/lorem-ipsum")]
    LoremIpsum {},
    #[route("/qr-code")]
    QrCodeGenerator {},
    #[route("/uuid")]
    UuidGenerator {},
}

fn Index() -> Element {
    rsx! {
        crate::pages::home_page::WidgetGrid { category_filter: Some("Generator") }
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
            Self::LoremIpsum { .. } => Some(&lorem_ipsum::WIDGET_ENTRY),
            Self::QrCodeGenerator { .. } => Some(&qr_code_generator::WIDGET_ENTRY),
            Self::UuidGenerator { .. } => Some(&uuid_generator::WIDGET_ENTRY),
            _ => None,
        }
    }

    fn get_category_entry() -> &'static CategoryEntry {
        &CATEGORY_ENTRY
    }
}

impl Default for GeneratorRoute {
    fn default() -> Self {
        Self::Index {}
    }
}
