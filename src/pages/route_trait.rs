use dioxus_router::prelude::*;
use strum::IntoEnumIterator;

use super::WidgetEntry;

pub trait WidgetRoute: Routable + IntoEnumIterator + PartialEq + Clone {
    fn get_widget_routes() -> Vec<super::Route>;

    fn get_widgets() -> Vec<Self> {
        Self::iter()
            .filter(|route| route.get_widget_entry().is_some())
            .collect()
    }

    fn get_widget_title_string(&self) -> Option<&'static str> {
        Some(self.get_widget_entry()?.title)
    }

    fn get_widget_short_title_string(&self) -> Option<&'static str> {
        Some(self.get_widget_entry()?.short_title)
    }

    fn get_widget_description_string(&self) -> Option<&'static str> {
        Some(self.get_widget_entry()?.description)
    }

    fn get_widget_type_string() -> &'static str;

    fn get_widget_entry(&self) -> Option<&'static WidgetEntry>;
}