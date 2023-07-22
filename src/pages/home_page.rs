#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsHouseDoorFill;
use dioxus_router::Link;

use crate::pages::{WidgetEntry, WidgetIcon, WIDGETS};

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
    cx.render(rsx! {
        div {
            class: "home-page",
            for widget_type in WIDGETS.keys() {
                for widget_entry in WIDGETS.get(widget_type).unwrap() {
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
                                to: widget_entry.path
                            }
                        }
                    }
                }
            }
        }
    })
}
