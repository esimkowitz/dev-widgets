#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaHouseChimney;
use strum::IntoEnumIterator;

use crate::pages::{CategoryEntry, Route, WidgetIcon};

pub static HOME_PAGE_CATEGORY_ENTRY: CategoryEntry = CategoryEntry {
    title: "Home",
    description: "Home page",
    icon: || HOME_ICON.icon(),
};

const HOME_ICON: WidgetIcon<FaHouseChimney> = WidgetIcon {
    icon: FaHouseChimney,
};

#[component]
pub fn WidgetGrid(category_filter: Option<&'static str>) -> Element {
    rsx! {
        div { class: "home-page",
            for route in Route::iter() {
                if category_filter.is_none() || route.get_widget_type_string() == category_filter {
                    for widget_route in route.get_widgets() {
                        if let Some(widget_entry) = widget_route.clone().get_widget_entry() {
                            div { class: "card card-widget bg-base-100",
                                figure { class: "card-figure", {(widget_entry.icon)()} }
                                div { class: "card-body",
                                    h2 { class: "card-title", {widget_entry.title} }
                                    p { {widget_entry.description} }
                                    Link {
                                        class: "absolute inset-0",
                                        to: widget_route,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn HomePage() -> Element {
    rsx! {
        WidgetGrid { category_filter: None }
    }
}
