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
    path: "/home",
    function: HomePage,
    icon: |cx| HOME_ICON.icon(cx),
};

const HOME_ICON: WidgetIcon<BsHouseDoorFill> = WidgetIcon {
    icon: BsHouseDoorFill,
};

pub fn HomePage(cx: Scope) -> Element {
    render! {
        div {
            class: "home-page",
            for route in Route::iter() {
                if let Some(widget_entry) = route.clone().get_widget_entry() {
                    rsx! {
                        div {
                            class: "card",
                            div {
                                class: "card-img-top",
                                (widget_entry.icon)(cx)
                            }
                            div {
                                class: "card-body",
                                div {
                                    class: "card-title",
                                    widget_entry.title
                                }
                                div {
                                    class: "card-text",
                                    widget_entry.description
                                }
                                Link {
                                    class: "stretched-link",
                                    to: route
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
