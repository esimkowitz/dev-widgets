// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::{Link, Redirect, Route, Router};

use phf::phf_ordered_map;

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
                        div {
                            class: "list-group-item list-group-item-action",
                            "Home"
                            Link {
                                class: "stretched-link",
                                to: "/home"
                            }
                        }
                        for widget_type in WIDGETS.keys() {
                            details {
                                class: "list-group-item pe-0",
                                summary {
                                    class: "section-header",
                                    *widget_type
                                }
                                for widget_entry in WIDGETS.get(widget_type).unwrap() {
                                    div {
                                        class: "list-group-item list-group-item-action m-0",
                                        widget_entry.title
                                        Link {
                                            class: "stretched-link",
                                            to: widget_entry.path
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "widget-view",
                        Route { to: "/home", home_page {} }
                        for widget_type in WIDGETS.keys() {
                            for widget_entry in WIDGETS.get(widget_type).unwrap() {
                                Route { to: widget_entry.path, (widget_entry.function)(cx) }
                            }
                        }
                    }
                }
                Redirect { from: "", to: "/home" }
            }
        }
    })
}

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
