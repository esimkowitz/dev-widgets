// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::{Router, Route, Link, Redirect, use_route};

use phf::phf_ordered_map;
use widget_entry::WidgetEntry;

pub mod base64_encoder;
pub mod color_picker;
pub mod date_converter;
pub mod json_yaml_converter;
pub mod number_base_converter;
pub mod widget_entry;

static WIDGETS: phf::OrderedMap<&str, &'static [widget_entry::WidgetEntry]> = phf_ordered_map! {
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
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        app,
        Config::default()
            .with_custom_head(
                r#"
                <link rel="stylesheet" href="../style/bootstrap.min.css">
                <link rel="stylesheet" href="../style/style.css">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Dev Widgets</title>
                "#
                .to_string(),
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
            class: "container-fluid",
            Router {
                div {
                    class: "d-flex flex-row wrapper",
                    div {
                        class: "list-group sidebar-list ms-2 mb-2 pt-2 pe-3 fixed-top",
                        sidebar_list_item {
                            widget_entry: HOME_PAGE_WIDGET_ENTRY
                        }
                        for widget_type in WIDGETS.keys() {
                            details {
                                class: "list-group-item pe-0",
                                open: true,
                                summary {
                                    class: "section-header",
                                    *widget_type
                                }
                                for widget_entry in WIDGETS.get(widget_type).unwrap() {
                                    sidebar_list_item {
                                        widget_entry: *widget_entry
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "widget-view",
                        Route { to: HOME_PAGE_WIDGET_ENTRY.path , (HOME_PAGE_WIDGET_ENTRY.function)(cx) }
                        for widget_type in WIDGETS.keys() {
                            for widget_entry in WIDGETS.get(widget_type).unwrap() {
                                Route { to: widget_entry.path, (widget_entry.function)(cx) }
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
fn sidebar_list_item(cx: Scope, widget_entry: WidgetEntry) -> Element {
    let route = use_route(cx);

    let active_str = if route.url().path() == widget_entry.path {
        "active"
    } else {
        ""
    };

    cx.render(rsx! {
        div {
            class: "list-group-item list-group-item-action {active_str}",
            widget_entry.title
            Link {
                class: "stretched-link",
                to: widget_entry.path
            }
        }
    })
}

static HOME_PAGE_WIDGET_ENTRY: widget_entry::WidgetEntry = widget_entry::WidgetEntry {
    title: "Home",
    description: "Home page",
    path: "/home",
    function: home_page,
};

fn home_page(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "pb-5 m-0 home-page",
            div {
                class: "widget-title",
                "Home"
            }

            div {
                class: "d-flex flex-row flex-wrap gap-2 widget-body",
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
        }
    })
}
