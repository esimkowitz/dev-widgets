use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsChevronLeft, BsChevronRight};
use dioxus_free_icons::Icon;
use dioxus_sdk::storage::use_persistent;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::components;
use crate::pages::home_page::HOME_PAGE_WIDGET_ENTRY;
use crate::pages::Route;

/// Sidebar state that persists to localStorage
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SidebarState {
    pub is_collapsed: bool,
    pub width: f32,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            is_collapsed: false,
            width: 15.0,
        }
    }
}

const MIN_WIDTH_EXPANDED: f32 = 12.0;
const MAX_WIDTH: f32 = 25.0;
const COLLAPSED_WIDTH: f32 = 4.0;

pub fn Container() -> Element {
    let mut sidebar_state = use_persistent("dev-widgets-sidebar", SidebarState::default);

    // Resizing state
    let mut is_resizing = use_signal(|| false);
    let resize_start_x = use_signal(|| 0.0f64);
    let resize_start_width = use_signal(|| 0.0f32);
    let mut drag_width = use_signal(|| 0.0f32); // Local signal for smooth drag updates

    let state = *sidebar_state.read();
    let current_width = if state.is_collapsed {
        COLLAPSED_WIDTH
    } else if *is_resizing.read() {
        *drag_width.read()
    } else {
        state.width
    };

    // Mouse move handler for resizing - updates local signal only (no localStorage writes)
    let onmousemove = move |evt: MouseEvent| {
        if *is_resizing.read() {
            let delta = (evt.client_coordinates().x - *resize_start_x.read()) as f32;
            let new_width = *resize_start_width.read() + (delta / 16.0); // Convert px to em (approx)
            drag_width.set(new_width.clamp(MIN_WIDTH_EXPANDED, MAX_WIDTH));
        }
    };

    // Mouse up handler - commits to persistent store only once
    let onmouseup = move |_: MouseEvent| {
        if *is_resizing.read() {
            sidebar_state.with_mut(|s| {
                s.width = *drag_width.read();
            });
        }
        is_resizing.set(false);
    };

    rsx! {
        div {
            class: "app-layout",
            onmousemove: onmousemove,
            onmouseup: onmouseup,

            // Persistent sidebar
            Sidebar {
                state: sidebar_state,
                is_resizing,
                resize_start_x,
                resize_start_width,
                current_width,
            }

            // Main content area with header
            div { class: "main-content",
                ContentHeader {}
                div { class: "content-body",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

/// Header showing current widget name
#[component]
fn ContentHeader() -> Element {
    let route = use_route::<Route>();
    let title = route
        .get_widget_entry()
        .map(|e| e.title)
        .unwrap_or("Home");

    rsx! {
        header { class: "content-header",
            h1 { class: "content-title", "{title}" }
        }
    }
}

#[component]
fn Sidebar(
    state: Signal<SidebarState>,
    is_resizing: Signal<bool>,
    resize_start_x: Signal<f64>,
    resize_start_width: Signal<f32>,
    current_width: f32,
) -> Element {
    let sidebar_state = *state.read();
    let is_collapsed = sidebar_state.is_collapsed;
    let resizing = *is_resizing.read();

    let sidebar_class = match (is_collapsed, resizing) {
        (true, _) => "sidebar collapsed",
        (false, true) => "sidebar expanded resizing",
        (false, false) => "sidebar expanded",
    };

    let menu_class = if is_collapsed {
        "sidebar-menu collapsed"
    } else {
        "sidebar-menu"
    };

    let width_style = format!("width: {}em;", current_width);

    rsx! {
        aside {
            class: "{sidebar_class}",
            style: "{width_style}",

            // Sidebar menu content
            ul { class: "{menu_class}",
                // Home link
                SidebarListItem {
                    widget_route: Route::HomePage {},
                    widget_entry_title: HOME_PAGE_WIDGET_ENTRY.short_title,
                    icon: (HOME_PAGE_WIDGET_ENTRY.icon)(),
                    is_collapsed,
                }

                // Category accordions
                for widget_type_route in Route::iter() {
                    if let Some(widget_type_string) = widget_type_route.get_widget_type_string() {
                        {
                            let widgets = widget_type_route.get_widgets();
                            let first_icon = widgets.first()
                                .and_then(|r| r.clone().get_widget_entry())
                                .map(|e| (e.icon)());

                            rsx! {
                                CollapsibleCategory {
                                    title: widget_type_string,
                                    is_sidebar_collapsed: is_collapsed,
                                    category_icon: first_icon,
                                    children: rsx! {
                                        for widget_route in widgets {
                                            if let Some(widget_entry) = widget_route.clone().get_widget_entry() {
                                                SidebarListItem {
                                                    widget_route,
                                                    widget_entry_title: widget_entry.short_title,
                                                    icon: (widget_entry.icon)(),
                                                    is_collapsed,
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

            // Collapse toggle button on the edge
            CollapseToggleButton { state }

            // Resize handle (only when expanded)
            if !is_collapsed {
                ResizeHandle {
                    state,
                    is_resizing,
                    resize_start_x,
                    resize_start_width,
                }
            }
        }
    }
}

#[component]
fn CollapseToggleButton(state: Signal<SidebarState>) -> Element {
    let is_collapsed = state.read().is_collapsed;

    let toggle = move |_| {
        state.with_mut(|s| {
            s.is_collapsed = !s.is_collapsed;
        });
    };

    rsx! {
        button {
            class: "collapse-toggle-btn",
            onclick: toggle,
            "aria-label": if is_collapsed { "Expand sidebar" } else { "Collapse sidebar" },
            if is_collapsed {
                Icon::<BsChevronRight> { icon: BsChevronRight, class: "w-4 h-4" }
            } else {
                Icon::<BsChevronLeft> { icon: BsChevronLeft, class: "w-4 h-4" }
            }
        }
    }
}

#[component]
fn ResizeHandle(
    state: Signal<SidebarState>,
    is_resizing: Signal<bool>,
    resize_start_x: Signal<f64>,
    resize_start_width: Signal<f32>,
) -> Element {
    let onmousedown = move |evt: MouseEvent| {
        evt.prevent_default();
        is_resizing.set(true);
        resize_start_x.set(evt.client_coordinates().x);
        resize_start_width.set(state.read().width);
    };

    rsx! {
        div {
            class: "resize-handle",
            onmousedown: onmousedown,
            "aria-label": "Resize sidebar"
        }
    }
}

#[component]
fn CollapsibleCategory(
    children: Element,
    title: &'static str,
    is_sidebar_collapsed: bool,
    category_icon: Option<Element>,
) -> Element {
    if is_sidebar_collapsed {
        // Collapsed sidebar: show icon with tooltip
        rsx! {
            li {
                class: "tooltip tooltip-right",
                "data-tip": "{title}",
                details { class: "collapse",
                    summary { class: "category-header collapsed",
                        if let Some(icon) = category_icon {
                            {icon}
                        }
                    }
                    ul { class: "menu menu-sm bg-base-200 rounded-box",
                        {children}
                    }
                }
            }
        }
    } else {
        // Expanded sidebar: use accordion
        rsx! {
            li {
                components::accordion::Accordion {
                    title: "{title}",
                    is_open: true,
                    {children}
                }
            }
        }
    }
}

#[component]
fn SidebarListItem(
    widget_route: Route,
    widget_entry_title: &'static str,
    icon: Element,
    is_collapsed: bool,
) -> Element {
    let route = use_route::<Route>();
    let is_active = widget_route == route;
    let active_class = if is_active { "menu-active" } else { "" };

    if is_collapsed {
        rsx! {
            li {
                class: "tooltip tooltip-right",
                "data-tip": "{widget_entry_title}",
                Link {
                    class: "{active_class}",
                    to: widget_route.clone(),
                    {icon}
                }
            }
        }
    } else {
        rsx! {
            li {
                Link {
                    class: "{active_class}",
                    to: widget_route.clone(),
                    {icon}
                    span { "{widget_entry_title}" }
                }
            }
        }
    }
}

#[component]
pub fn WidgetView() -> Element {
    rsx! {
        div { class: "widget-view",
            Outlet::<Route> {}
        }
    }
}
