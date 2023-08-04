
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components;
use crate::pages::home_page::HOME_PAGE_WIDGET_ENTRY;
use crate::pages::Route;

pub fn Container(cx: Scope) -> Element {
    render! {
        div {
            class: "container-fluid",
            Sidebar {}
            Outlet::<Route> { }
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
                        widget_route: Route::HomePage {},
                        widget_entry_title: HOME_PAGE_WIDGET_ENTRY.short_title,
                        icon: (HOME_PAGE_WIDGET_ENTRY.icon)(cx)
                    }
                    for widget_type_route in Route::get_widget_types() {
                        log::info!("widget_type_route: {:?}", widget_type_route)
                        components::accordion::Accordion {
                            title: "{widget_type_route.get_widget_type_string().unwrap()}",
                            is_open: true,
                            for widget_route in Route::get_widget_routes_for_type(widget_type_route) {
                                if let Some(widget_entry) = widget_route.clone().get_widget_entry() {
                                    rsx! {
                                        SidebarListItem {
                                            widget_route: widget_route,
                                            widget_entry_title: widget_entry.short_title,
                                            icon: (widget_entry.icon)(cx)
                                        }
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
fn SidebarListItem<'a>(cx: Scope<'a>, widget_route: Route, widget_entry_title: &'a str, icon: Element<'a>) -> Element<'a> {
    let route = use_route::<Route>(cx).unwrap();

    let active_str = if widget_route == &route {
        "active"
    } else {
        ""
    };

    render! {
        Link {
            class: "btn {active_str}",
            to: widget_route.clone(),
            icon
            "{widget_entry_title}"
        }
    }
}


#[inline_props]
pub fn WidgetView(cx: Scope) -> Element {
    let route = use_route::<Route>(cx).unwrap();
    let mut title = "Home";
    if let Some(widget_entry) = route.get_widget_entry() {
        title = widget_entry.title;
    }
    render! {
        div {
            class: "widget-view",
            h3 {
                class: "widget-title",
                "{title}"
            }
            div {
                class: "widget-body",
                Outlet::<Route> { }
            }
        }
    }
}