#![allow(non_snake_case)]
pub mod components;
pub mod environment;
pub mod pages;
pub mod utils;

use dioxus::prelude::*;
use dioxus_router::{use_route, Link, Redirect, Route, Router};

use crate::pages::home_page::HOME_PAGE_WIDGET_ENTRY;
use crate::pages::{ WidgetEntry, WIDGETS };

pub fn App(cx: Scope) -> Element {
    render! {
        div {
            class: "container-fluid",
            Router {
                Sidebar {}
                div {
                    class: "widget-view",
                    Route {
                        to: HOME_PAGE_WIDGET_ENTRY.path,
                        WidgetView {
                            title: HOME_PAGE_WIDGET_ENTRY.title,
                            children: (HOME_PAGE_WIDGET_ENTRY.function)(cx)
                        }
                    }
                    for widget_type in WIDGETS.keys() {
                        for widget_entry in WIDGETS.get(widget_type).unwrap() {
                            Route {
                                to: widget_entry.path,
                                WidgetView {
                                    title: widget_entry.title,
                                    children: (widget_entry.function)(cx)
                                }
                            }
                        }
                    }
                }
                Redirect { from: "", to: HOME_PAGE_WIDGET_ENTRY.path }
            }
        }
    }
}

fn Sidebar(cx: Scope) -> Element {
    render! {
        div {
            class: "sidebar",
            div {
                class: "sidebar-list",
                div {
                    class: "accordion",
                    SidebarListItem {
                        widget_entry: HOME_PAGE_WIDGET_ENTRY,
                        icon: (HOME_PAGE_WIDGET_ENTRY.icon)(cx)
                    }
                    for widget_type in WIDGETS.keys() {
                        div {
                            components::accordion::Accordion {
                                title: *widget_type,
                                is_open: true,
                                for widget_entry in WIDGETS.get(widget_type).unwrap() {
                                    SidebarListItem {
                                        widget_entry: *widget_entry,
                                        icon: (widget_entry.icon)(cx)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "vr"
            }
        }
    }
}

#[inline_props]
fn WidgetView<'a>(cx: Scope<'a>, children: Element<'a>, title: &'a str) -> Element {
    render! {
        h3 {
            class: "widget-title",
            *title
        }
        div {
            class: "widget-body",
            children
        }
    }
}

#[inline_props]
fn SidebarListItem<'a>(cx: Scope<'a>, widget_entry: WidgetEntry, icon: Element<'a>) -> Element {
    let route = use_route(cx);

    let active_str = if route.url().path() == widget_entry.path {
        "active"
    } else {
        ""
    };

    render! {
        Link {
            class: "btn {active_str}",
            to: widget_entry.path,
            icon
            widget_entry.short_title
        }
    }
}
