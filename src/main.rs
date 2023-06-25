#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{Config as DesktopConfig, WindowBuilder};
use dioxus_free_icons::icons::bs_icons::BsHouseDoorFill;
use dioxus_router::{use_route, Link, Redirect, Route, Router};

#[cfg(debug_assertions)]
use dioxus_hot_reload::{hot_reload_init, Config as HotReloadConfig};
use std::{env, time::SystemTime, alloc::System};

use phf::phf_ordered_map;
use sidebar_icon::SidebarIcon;
use widget_entry::WidgetEntry;

pub mod accordion;
pub mod base64_encoder;
pub mod color_picker;
pub mod date_converter;
pub mod json_yaml_converter;
pub mod number_base_converter;
pub mod sidebar_icon;
pub mod widget_entry;

static WIDGETS: phf::OrderedMap<&str, &'static [WidgetEntry]> = phf_ordered_map! {
    "Encoder" => &[
        base64_encoder::WIDGET_ENTRY,
    ],
    "Converter" => &[
        number_base_converter::WIDGET_ENTRY,
        date_converter::WIDGET_ENTRY,
        json_yaml_converter::WIDGET_ENTRY,
    ],
    "Media" => &[
        color_picker::WIDGET_ENTRY,
    ],
};

fn main() {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");

        hot_reload_init!(HotReloadConfig::new()
            .with_paths(&["src", "style", "scss"])
            .with_rebuild_command("cargo run"));
    }
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        App,
        DesktopConfig::default()
            .with_custom_index(
                r#"
                    <!DOCTYPE html>
                    <html data-bs-theme="light">
                        <head>
                            <title>Dev Widgets</title>
                            <link rel="stylesheet" href="../style/style.css">
                            <meta name="viewport" content="width=device-width, initial-scale=1">
                        </head>
                        <body>
                            <div id="main"></div>
                            <script type="text/javascript">
                                // Set theme to the user's preferred color scheme
                                function updateTheme() {
                                const colorMode = window.matchMedia("(prefers-color-scheme: dark)").matches ?
                                    "dark" :
                                    "light";
                                document.querySelector("html").setAttribute("data-bs-theme", colorMode);
                                }

                                // Set theme on load
                                updateTheme()

                                // Update theme when the preferred scheme changes
                                window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', updateTheme)
                            </script>
                            <script type="text/javascript" src="../js/bootstrap.min.js"></script>
                        </body>
                    </html>
                "#.to_string()
            )
            .with_window(
                WindowBuilder::new()
                    .with_title("Dev Widgets")
                    .with_resizable(true)
                    .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(
                        800.0, 800.0,
                    ))
                    .with_min_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(
                        600.0, 300.0,
                    )),
            ),
    );
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SidebarState {
        is_tracking: false,
        start_width: 0.0,
        start_screen_x: 0.0,
        current_width: 230.0,
        max_width: 500.0,
        min_width: 1.0,
        last_event_time: SystemTime::now(),
    });
    let sidebar_state = use_shared_state::<SidebarState>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: "container-fluid",
            onmousemove: |event| {
                event.stop_propagation();
                if sidebar_state.read().is_tracking && sidebar_state.read().last_event_time.elapsed().unwrap().as_millis() > 10 {
                    let mut state = sidebar_state.read().clone();
                    let screen_x_delta = event.screen_coordinates().x - state.start_screen_x;
                    let new_width = f64::min(state.current_width + screen_x_delta, state.max_width);
                    state.current_width = f64::max(new_width, state.min_width);
                    state.last_event_time = SystemTime::now();
                    *sidebar_state.write() = state;
                }
            },
            onmouseup: move |event| {
                event.stop_propagation();
                if sidebar_state.read().is_tracking {
                    sidebar_state.write().is_tracking = false;
                }
                println!("mouseup");
            },
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
                        div {
                            class: "list-unstyled",
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
                }
                Redirect { from: "", to: HOME_PAGE_WIDGET_ENTRY.path }
            }
        }
    })
}

fn Sidebar(cx: Scope) -> Element {
    let sidebar_state = use_shared_state::<SidebarState>(cx).unwrap();

    let current_state = sidebar_state.read().clone();
    let current_width = current_state.current_width;
    let sidebar_css = if current_width == current_state.min_width {
        format!("width: {current_width}px; cursor: e-resize;")
    } else if current_width == current_state.max_width {
        format!("width: {current_width}px; cursor: w-resize;")
    } else {
        format!("width: {current_width}px; cursor: col-resize;")
    };

    cx.render(rsx! {
        div {
            class: "sidebar",
            style: "{sidebar_css}",
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
                            accordion::Accordion {
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
                class: "vr",
                onmousedown: move |event| {
                    event.stop_propagation();
                    let mut current_state = sidebar_state.read().clone();
                    current_state.is_tracking = true;
                    current_state.start_width = current_state.current_width;
                    current_state.start_screen_x = event.screen_coordinates().x;
                    *sidebar_state.write() = current_state;
                    println!("mousedown");
                },
            }
        }
    })
}

#[derive(Clone, Copy, Debug)]
struct SidebarState {
    is_tracking: bool,
    start_width: f64,
    start_screen_x: f64,
    current_width: f64,
    max_width: f64,
    min_width: f64,
    last_event_time: SystemTime,
}

#[inline_props]
fn WidgetView<'a>(cx: Scope<'a>, children: Element<'a>, title: &'a str) -> Element {
    cx.render(rsx! {
        h3 {
            class: "widget-title",
            *title
        }
        div {
            class: "widget-body",
            children
        }
    })
}

#[inline_props]
fn SidebarListItem<'a>(cx: Scope<'a>, widget_entry: WidgetEntry, icon: Element<'a>) -> Element {
    let route = use_route(cx);

    let active_str = if route.url().path() == widget_entry.path {
        "active"
    } else {
        ""
    };

    cx.render(rsx! {
        Link {
            class: "btn {active_str}",
            to: widget_entry.path
            icon
            widget_entry.short_title
        }
    })
}

static HOME_PAGE_WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Home",
    short_title: "Home",
    description: "Home page",
    path: "/home",
    function: HomePage,
    icon: |cx| HOME_SIDEBAR_ICON.sidebar_icon(cx),
};

fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "home-page",
            for widget_type in WIDGETS.keys() {
                for widget_entry in WIDGETS.get(widget_type).unwrap() {
                    div {
                        class: "card",
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

const HOME_SIDEBAR_ICON: SidebarIcon<BsHouseDoorFill> = SidebarIcon {
    icon: BsHouseDoorFill,
};
