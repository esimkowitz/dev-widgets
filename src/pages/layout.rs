use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::components;
use crate::pages::home_page::HOME_PAGE_WIDGET_ENTRY;
use crate::pages::Route;

pub fn Container() -> Element {
    rsx! {
        div { class: "container-fluid",
            Sidebar {}
            Outlet::<Route> {}
        }
    }
}

fn Sidebar() -> Element {
    rsx! {
        div { class: "sidebar",
            div { class: "sidebar-list",
                div { class: "accordion",
                    SidebarListItem {
                        widget_route: Route::HomePage {},
                        widget_entry_title: HOME_PAGE_WIDGET_ENTRY.short_title,
                        icon: (HOME_PAGE_WIDGET_ENTRY.icon)(),
                    }
                    for widget_type_route in Route::iter() {
                        if let Some(widget_type_string) = widget_type_route.get_widget_type_string() {
                            {
                                rsx! {
                                    components::accordion::Accordion { title: "{widget_type_string}", is_open: true,
                                        for widget_route in widget_type_route.get_widgets() {
                                            if let Some(widget_entry) = widget_route.clone().get_widget_entry() {
                                                {
                                                    rsx! {
                                                        SidebarListItem {
                                                            widget_route,
                                                            widget_entry_title: widget_entry.short_title,
                                                            icon: (widget_entry.icon)(),
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
                }
            }
            div { class: "vr" }
        }
    }
}

#[component]
fn SidebarListItem(
    widget_route: Route,
    widget_entry_title: &'static str,
    icon: Element,
) -> Element {
    let route = use_route::<Route>();

    let active_str = if widget_route == route { "active" } else { "" };

    rsx! {
        Link { class: "btn {active_str}", to: widget_route.clone(),
            {icon}
            "{widget_entry_title}"
        }
    }
}

#[component]
pub fn WidgetView() -> Element {
    let route = use_route::<Route>();
    let mut title = "Home";
    if let Some(widget_entry) = route.get_widget_entry() {
        title = widget_entry.title;
    }
    rsx! {
        div { class: "widget-view",
            h3 { class: "widget-title", "{title}" }
            div { class: "widget-body", Outlet::<Route> {} }
        }
    }
}
