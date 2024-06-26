#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsHouseDoorFill;
use strum::IntoEnumIterator;

use crate::pages::{Route, WidgetEntry, WidgetIcon};

pub static HOME_PAGE_WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Home",
    short_title: "Home",
    description: "Home page",
    icon: || HOME_ICON.icon(),
};

const HOME_ICON: WidgetIcon<BsHouseDoorFill> = WidgetIcon {
    icon: BsHouseDoorFill,
};

pub fn HomePage() -> Element {
    rsx! {
        div {
            class: "home-page",
            for route in Route::iter() {
                for widget_route in route.get_widgets() {
                    if let Some(widget_entry) = widget_route.clone().get_widget_entry() {
                        {rsx! {
                            div {
                                class: "card",
                                div {
                                    class: "card-img-top",
                                    {(widget_entry.icon)()}
                                }
                                div {
                                    class: "card-body",
                                    div {
                                        class: "card-title",
                                        {widget_entry.title}
                                    }
                                    div {
                                        class: "card-text",
                                        {widget_entry.description}
                                    }
                                    Link {
                                        class: "stretched-link",
                                        to: widget_route
                                    }
                                }
                            }
                        }}
                    }
                }
            }
        }
    }
}
