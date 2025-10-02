use dioxus::prelude::*;
use dioxus_router::prelude::*;
use strum_macros::EnumIter;

pub mod color_picker;

use crate::pages::{Route, WidgetEntry, WidgetRoute};
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
        div { class: "media" }
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
}

impl Default for MediaRoute {
    fn default() -> Self {
        Self::Index {}
    }
}
