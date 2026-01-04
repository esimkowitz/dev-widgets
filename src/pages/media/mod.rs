use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaPhotoFilm;
use dioxus_free_icons::Icon;
use strum_macros::EnumIter;

pub mod color_picker;

use crate::pages::{CategoryEntry, Route, WidgetEntry, WidgetRoute};

pub static CATEGORY_ENTRY: CategoryEntry = CategoryEntry {
    title: "Media",
    description: "Tools for working with colors and media",
    icon: || {
        rsx! {
            Icon::<FaPhotoFilm> { class: "icon", icon: FaPhotoFilm }
        }
    },
};
use color_picker::ColorPicker;

#[derive(Clone, Debug, EnumIter, PartialEq, Routable)]
pub enum MediaRoute {
    #[route("/")]
    Index {},
    #[route("/color-picker")]
    ColorPicker {},
}

fn Index() -> Element {
    rsx! {
        crate::pages::home_page::WidgetGrid { category_filter: Some("Media") }
    }
}

impl WidgetRoute for MediaRoute {
    fn get_widget_routes() -> Vec<Route> {
        Self::get_widgets()
            .iter()
            .map(|widget| Route::Media {
                child: widget.clone(),
            })
            .collect()
    }

    fn get_widget_type_string() -> &'static str {
        "Media"
    }

    fn get_widget_entry(&self) -> Option<&'static WidgetEntry> {
        match self {
            Self::ColorPicker { .. } => Some(&color_picker::WIDGET_ENTRY),
            _ => None,
        }
    }

    fn get_category_entry() -> &'static CategoryEntry {
        &CATEGORY_ENTRY
    }
}

impl Default for MediaRoute {
    fn default() -> Self {
        Self::Index {}
    }
}
