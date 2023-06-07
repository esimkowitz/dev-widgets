// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use phf::phf_map;

pub mod base64_encoder;
pub mod date_converter;
pub mod json_yaml_converter;
pub mod number_base_converter;

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

static WIDGETS: phf::Map<&str, &'static [WidgetEntry]> = phf_map! {
    "Encoder" => &[
        WidgetEntry {
            title: base64_encoder::TITLE,
            description: base64_encoder::DESCRIPTION,
            widget_type: WidgetType::Encoder,
            widget: Widget::Base64Encoder,
        },
    ],
    "Converter" => &[
        WidgetEntry {
            title: number_base_converter::TITLE,
            description: number_base_converter::DESCRIPTION,
            widget_type: WidgetType::Converter,
            widget: Widget::NumberBaseConverter,
        },
        WidgetEntry {
            title: date_converter::TITLE,
            description: date_converter::DESCRIPTION,
            widget_type: WidgetType::Converter,
            widget: Widget::DateConverter,
        },
        WidgetEntry {
            title: json_yaml_converter::TITLE,
            description: json_yaml_converter::DESCRIPTION,
            widget_type: WidgetType::Converter,
            widget: Widget::JsonYamlConverter,
        },
    ],
};

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || WidgetViewState {
        current_widget: Widget::Home,
    });
    let state = use_shared_state::<WidgetViewState>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "container-fluid",
            div {
                class: "d-flex flex-row wrapper",
                div {
                    class: "list-group sidebar-list ms-2 mb-2 pt-2 pe-3 fixed-top",
                    a {
                        class: "list-group-item list-group-item-action",
                        onclick: move |_| state.write().current_widget = Widget::Home,
                        "Home"
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
                                    onclick: move |_| state.write().current_widget = widget_entry.widget,
                                    a {
                                        widget_entry.title
                                    }
                                }
                            }
                        }
                    }
                }
                widget_view {}
            }
        }
    })
}

fn widget_view(cx: Scope) -> Element {
    let state = use_shared_state::<WidgetViewState>(cx).unwrap();

    fn set_display(current_widget: Widget, desired_widget: Widget) -> &'static str {
        if current_widget == desired_widget {
            "block"
        } else {
            "none"
        }
    }
    cx.render(rsx! {
        div {
            class: "widget-view",
            div {
                display: set_display(state.read().current_widget, Widget::Home),
                home_page {}
            }
            div {
                display: set_display(state.read().current_widget, Widget::Base64Encoder),
                base64_encoder::base64_encoder {}
            }
            div {
                display: set_display(state.read().current_widget, Widget::NumberBaseConverter),
                number_base_converter::number_base_converter {}
            }
            div {
                display: set_display(state.read().current_widget, Widget::DateConverter),
                date_converter::date_converter {}
            }
            div {
                display: set_display(state.read().current_widget, Widget::JsonYamlConverter),
                json_yaml_converter::json_yaml_converter {}
            }
        }
    })
}

fn home_page(cx: Scope) -> Element {
    let state = use_shared_state::<WidgetViewState>(cx).unwrap();
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
                            onclick: move |_| state.write().current_widget = widget_entry.widget,
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
                            }
                        }
                    }
                }
            }
        }
    })
}

struct WidgetViewState {
    current_widget: Widget,
}

#[derive(PartialEq, Eq)]
struct WidgetEntry {
    title: &'static str,
    description: &'static str,
    widget_type: WidgetType,
    widget: Widget,
}

#[derive(PartialEq, Eq, Hash)]
enum WidgetType {
    Converter,
    Encoder,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Widget {
    Base64Encoder,
    DateConverter,
    NumberBaseConverter,
    JsonYamlConverter,
    Home,
}
