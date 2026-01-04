use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::fa_regular_icons::FaCopyright;
use dioxus_free_icons::icons::fa_solid_icons::{FaChevronLeft, FaChevronRight};
use dioxus_free_icons::Icon;
use dioxus_sdk::storage::use_persistent;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use time::OffsetDateTime;

use crate::components;
use crate::pages::home_page::HOME_PAGE_CATEGORY_ENTRY;
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

const MIN_WIDTH_EXPANDED: f32 = 9.0;
const COLLAPSE_THRESHOLD: f32 = 8.0;
const EXPAND_THRESHOLD: f32 = 5.0;
const MAX_WIDTH: f32 = 25.0;
const COLLAPSED_WIDTH: f32 = 4.0;

pub fn Container() -> Element {
    let mut sidebar_state = use_persistent("dev-widgets-sidebar", SidebarState::default);

    // Resizing state
    let mut is_resizing = use_signal(|| false);
    let mut resize_start_x = use_signal(|| 0.0f64);
    let mut resize_start_width = use_signal(|| 0.0f32);
    let mut drag_width = use_signal(|| 0.0f32); // Local signal for smooth drag updates

    let state = *sidebar_state.read();
    let current_width = if state.is_collapsed {
        COLLAPSED_WIDTH
    } else if *is_resizing.read() {
        *drag_width.read()
    } else {
        state.width
    };

    // Pointer move handler for resizing with collapse/expand thresholds
    //
    // Key insight: After toggling, we reset the drag origin to the current mouse position
    // and set resize_start_width to the new state's width. This means:
    // - After collapse: start_width = COLLAPSED_WIDTH (4em), so user must drag right to increase
    // - After expand: start_width = MIN_WIDTH_EXPANDED (9em), so user must drag left to decrease
    //
    // The threshold gap (collapse at 8em, expand at 5em) plus the origin reset means:
    // - To re-expand after collapse: must drag from 4em base past 5em = 1em+ rightward drag
    // - To re-collapse after expand: must drag from 9em base below 8em = 1em+ leftward drag
    let onpointermove = move |evt: PointerEvent| {
        if *is_resizing.read() {
            let delta = (evt.client_coordinates().x - *resize_start_x.read()) as f32;
            let new_width = *resize_start_width.read() + (delta / 16.0); // Convert px to em (approx)
            let is_collapsed = sidebar_state.read().is_collapsed;

            if is_collapsed {
                // Expand when dragging past expand threshold
                if new_width > EXPAND_THRESHOLD {
                    let clamped = new_width.clamp(MIN_WIDTH_EXPANDED, MAX_WIDTH);
                    drag_width.set(clamped);
                    sidebar_state.with_mut(|s| {
                        s.is_collapsed = false;
                        s.width = clamped;
                    });
                    // Reset origin: now at MIN_WIDTH_EXPANDED, must drag 1em+ left to collapse again
                    resize_start_x.set(evt.client_coordinates().x);
                    resize_start_width.set(clamped);
                }
            } else {
                // Collapse when dragging below collapse threshold
                if new_width < COLLAPSE_THRESHOLD {
                    sidebar_state.with_mut(|s| s.is_collapsed = true);
                    // Reset origin: now at COLLAPSED_WIDTH, must drag 1em+ right to expand again
                    resize_start_x.set(evt.client_coordinates().x);
                    resize_start_width.set(COLLAPSED_WIDTH);
                } else {
                    drag_width.set(new_width.clamp(MIN_WIDTH_EXPANDED, MAX_WIDTH));
                }
            }
        }
    };

    // Pointer up handler - commits to persistent store only once
    let onpointerup = move |_: PointerEvent| {
        if *is_resizing.read() {
            let width = *drag_width.read();
            // Only commit width if it's valid (user actually dragged)
            if width >= MIN_WIDTH_EXPANDED {
                sidebar_state.with_mut(|s| s.width = width);
            }
        }
        is_resizing.set(false);
    };

    rsx! {
        div { class: "app-layout", onpointermove, onpointerup,

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
                div { class: "content-body", Outlet::<Route> {} }
            }
        }
    }
}

/// Header showing current widget or category name
#[component]
fn ContentHeader() -> Element {
    let route = use_route::<Route>();
    let title = route
        .get_widget_entry()
        .map(|e| e.title)
        .or_else(|| route.get_category_entry().map(|e| e.title))
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
    let target_width = sidebar_state.width;

    let sidebar_class = match (is_collapsed, resizing) {
        (true, true) => "sidebar collapsed resizing",
        (true, false) => "sidebar collapsed",
        (false, true) => "sidebar expanded resizing",
        (false, false) => "sidebar expanded",
    };

    // Outer width animates, inner content uses appropriate width
    let outer_width_style = format!("width: {}em;", current_width);
    // When resizing: inner follows current_width for immediate feedback
    // When collapsed: no width needed
    // When expanded (not resizing): use target_width to prevent text reflow during collapse animation
    let inner_width_style = if is_collapsed {
        String::new()
    } else if resizing {
        format!("width: {}em;", current_width)
    } else {
        format!("width: {}em;", target_width)
    };

    rsx! {
        aside { class: "{sidebar_class}", style: "{outer_width_style}",

            // Always render both collapsed and expanded views
            // CSS handles visibility with delayed opacity transitions

            // Collapsed view: icon-only menu
            ul { class: "sidebar-menu sidebar-menu-collapsed",
                // Home (childless category)
                CollapsibleCategoryCollapsed {
                    title: HOME_PAGE_CATEGORY_ENTRY.title,
                    category_icon: Some((HOME_PAGE_CATEGORY_ENTRY.icon)()),
                    category_route: Route::HomePage {},
                }

                // Category icons
                for widget_type_route in Route::iter() {
                    if let Some(category_entry) = widget_type_route.get_category_entry() {
                        CollapsibleCategoryCollapsed {
                            title: category_entry.title,
                            category_icon: Some((category_entry.icon)()),
                            category_route: widget_type_route.clone(),
                        }
                    }
                }
            }

            // Expanded view: inner wrapper with fixed width prevents text reflow
            div {
                class: "sidebar-inner sidebar-inner-expanded",
                style: "{inner_width_style}",

                ul { class: "sidebar-menu",
                    // Home (childless category)
                    CollapsibleCategoryExpanded {
                        title: HOME_PAGE_CATEGORY_ENTRY.title,
                        category_icon: Some((HOME_PAGE_CATEGORY_ENTRY.icon)()),
                        category_route: Route::HomePage {},
                        has_children: false,
                    }

                    // Category accordions
                    for widget_type_route in Route::iter() {
                        if let Some(category_entry) = widget_type_route.get_category_entry() {
                            {
                                let widgets = widget_type_route.get_widgets();
                                let category_icon = Some((category_entry.icon)());
                                let category_index_route = widget_type_route.clone();

                                rsx! {
                                    CollapsibleCategoryExpanded {
                                        title: category_entry.title,
                                        category_icon,
                                        category_route: category_index_route,
                                        children: rsx! {
                                            for widget_route in widgets {
                                                if let Some(widget_entry) = widget_route.clone().get_widget_entry() {
                                                    SidebarWidgetItem {
                                                        widget_route,
                                                        widget_entry_title: widget_entry.short_title,
                                                        icon: (widget_entry.icon)(),
                                                    }
                                                }
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }

                SidebarFooter {}
            }

            // Collapse toggle button on the edge
            CollapseToggleButton { state }

            // Resize handle (always visible for drag-to-expand from collapsed)
            ResizeHandle {
                state,
                is_resizing,
                resize_start_x,
                resize_start_width,
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
                Icon::<FaChevronRight> { icon: FaChevronRight, class: "w-4 h-4" }
            } else {
                Icon::<FaChevronLeft> { icon: FaChevronLeft, class: "w-4 h-4" }
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
    let onpointerdown = move |evt: PointerEvent| {
        evt.prevent_default();
        is_resizing.set(true);
        resize_start_x.set(evt.client_coordinates().x);
        // When starting from collapsed, use COLLAPSED_WIDTH as the base
        let start_width = if state.read().is_collapsed {
            COLLAPSED_WIDTH
        } else {
            state.read().width
        };
        resize_start_width.set(start_width);
    };

    rsx! {
        div {
            class: "resize-handle",
            onpointerdown,
            "aria-label": "Resize sidebar",
        }
    }
}

/// Collapsed category: just an icon link with tooltip
#[component]
fn CollapsibleCategoryCollapsed(
    title: &'static str,
    category_icon: Option<Element>,
    category_route: Route,
) -> Element {
    rsx! {
        li {
            Link {
                class: "category-header collapsed",
                title: "{title}",
                to: category_route,
                if let Some(icon) = category_icon {
                    {icon}
                }
            }
        }
    }
}

/// Expanded category: accordion with optional children
#[component]
fn CollapsibleCategoryExpanded(
    title: &'static str,
    category_icon: Option<Element>,
    category_route: Route,
    #[props(default)] children: Element,
    #[props(default = true)] has_children: bool,
) -> Element {
    rsx! {
        li {
            components::accordion::Accordion {
                title: "{title}",
                icon: category_icon,
                is_open: true,
                category_route: Some(category_route),
                has_children,
                {children}
            }
        }
    }
}

#[component]
fn SidebarWidgetItem(
    widget_route: Route,
    widget_entry_title: &'static str,
    icon: Element,
) -> Element {
    let route = use_route::<Route>();
    let is_active = widget_route == route;
    let active_class = if is_active { "menu-active" } else { "" };

    // Widget items are always inside accordions (expanded view only)
    // They don't need collapsed versions since accordions are hidden when collapsed
    rsx! {
        li {
            Link { class: "{active_class}", to: widget_route.clone(),
                {icon}
                span { "{widget_entry_title}" }
            }
        }
    }
}

#[component]
pub fn WidgetView() -> Element {
    rsx! {
        div { class: "widget-view", Outlet::<Route> {} }
    }
}

#[component]
fn SidebarFooter() -> Element {
    let current_year = OffsetDateTime::now_utc().year();
    let version = env!("CARGO_PKG_VERSION");

    rsx! {
        div { class: "sidebar-footer",
            p { "Dev Widgets v{version}" }
            a {
                class: "link link-hover",
                href: "https://github.com/esimkowitz/dev-widgets",
                target: "_blank",
                Icon::<FaGithub> { icon: FaGithub, class: "inline-block w-3 h-3" }
                " esimkowitz/dev-widgets"
            }
            a {
                class: "link link-hover",
                href: "https://github.com/esimkowitz/dev-widgets/blob/main/LICENSE",
                target: "_blank",
                "Copyright "
                Icon::<FaCopyright> { icon: FaCopyright, class: "inline-block w-3 h-3" }
                " {current_year} Evan Simkowitz"
            }
        }
    }
}
