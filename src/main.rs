// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_desktop::{Config as DesktopConfig, WindowBuilder};
use dioxus_free_icons::{icons::bs_icons::{BsHouseDoorFill, BsHash}, IconShape, Icon};
use dioxus_router::{use_route, Link, Redirect, Route, Router};

#[cfg(debug_assertions)]
use dioxus_hot_reload::{hot_reload_init, Config as HotReloadConfig};

use phf::phf_ordered_map;
use sidebar_icon::SidebarIcon;
use widget_entry::WidgetEntry;

pub mod accordion;
pub mod base64_encoder;
pub mod color_picker;
pub mod date_converter;
pub mod json_yaml_converter;
pub mod number_base_converter;
pub mod widget_entry;
pub mod sidebar_icon;

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
        hot_reload_init!(HotReloadConfig::new()
            .with_paths(&["src", "style", "scss"])
            .with_rebuild_command("cargo run"));
    }
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        app,
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

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container-fluid d-flex flex-row wrapper pe-0",
            Router {
                div {
                    class: "sidebar-list",
                    div {
                        class: "accordion accordion-flush flex-column ms-2 mb-2 pt-2 pe-3",
                        sidebar_list_item {
                            widget_entry: HOME_PAGE_WIDGET_ENTRY,
                            icon: (HOME_PAGE_WIDGET_ENTRY.icon)(cx)
                        }
                        for widget_type in WIDGETS.keys() {
                            div {
                                accordion::accordion {
                                    title: *widget_type,
                                    for widget_entry in WIDGETS.get(widget_type).unwrap() {
                                        sidebar_list_item {
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
                    class: "widget-view",
                    Route {
                        to: HOME_PAGE_WIDGET_ENTRY.path,
                        widget_view {
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
                                    widget_view {
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

#[inline_props]
fn widget_view<'a>(cx: Scope<'a>, children: Element<'a>, title: &'a str) -> Element {
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
fn sidebar_list_item<'a>(cx: Scope<'a>, widget_entry: WidgetEntry, icon: Element<'a>) -> Element {
    let route = use_route(cx);

    let active_str = if route.url().path() == widget_entry.path {
        "active"
    } else {
        ""
    };

    cx.render(rsx! {
        Link {
            class: "btn btn-sm {active_str}",
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
    function: home_page,
    icon: |cx| HOME_SIDEBAR_ICON.sidebar_icon(cx),
};

fn home_page(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "home-page d-flex flex-row flex-wrap gap-2",
            for widget_type in WIDGETS.keys() {
                for widget_entry in WIDGETS.get(widget_type).unwrap() {
                    div {
                        class: "card p-0",
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

pub fn home_icon(cx: Scope) -> Element {
    cx.render(rsx! {
        Icon {
            class: "home-icon",
            icon: BsHouseDoorFill
        }
    })
}