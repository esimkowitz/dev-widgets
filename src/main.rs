// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use phf::phf_map;

pub mod base64_encoder;
pub mod number_base_converter;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        app,
        Config::default()
            .with_custom_head(
                r#"
                <link rel="stylesheet" href="../style/style.css">
                <link rel="stylesheet" href="../style/bootstrap.min.css">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                "#
                .to_string(),
            )
            .with_window(
                WindowBuilder::new()
                    .with_title("Dev Widgets")
                    .with_resizable(true)
                    .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(
                        800.0, 800.0,
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
        }
    ],
    "Converter" => &[
        WidgetEntry {
            title: number_base_converter::TITLE,
            description: number_base_converter::DESCRIPTION,
            widget_type: WidgetType::Converter,
            widget: Widget::NumberBaseConverter,
        }
    ],
};

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || WidgetViewState {
        current_widget: Widget::Home,
    });
    let state = use_shared_state::<WidgetViewState>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "container-fluid align-items-start",
            div {
                class: "row",
                div {
                    class: "col-3 list-group",
                    style: "width: 14em;",
                    a {
                        class: "list-group-item list-group-item-action",
                        onclick: move |_| state.write().current_widget = Widget::Home,
                        "Home"
                    }
                    ul {
                        class: "list-group",
                        for widget_type in WIDGETS.keys() {
                            details {
                                class: "list-group-item list-group",
                                summary {
                                    class: "section-header",
                                    *widget_type
                                }
                                for widget_entry in WIDGETS.get(widget_type).unwrap() {
                                    div {
                                        class: "list-group-item-action",                                            onclick: move |_| state.write().current_widget = widget_entry.widget,
                                        onclick: move |_| state.write().current_widget = widget_entry.widget,
                                        a {
                                            widget_entry.title
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "col-7",
                    widget_view {}
                }
            }
        }
    })
}

fn widget_view(cx: Scope) -> Element {
    let state = use_shared_state::<WidgetViewState>(cx).unwrap();

    fn set_display(current_widget: Widget, desired_widget: Widget) -> &'static str {
        if current_widget == desired_widget {
            "flex"
        } else {
            "none"
        }
    }
    cx.render(rsx! {
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
    })
}

fn home_page(cx: Scope) -> Element {
    let state = use_shared_state::<WidgetViewState>(cx).unwrap();
    cx.render(rsx! {
        div {
            h2 {
                "Home"
            }

            div {
                class: "row gap-2",
                for widget_type in WIDGETS.keys() {
                    for widget_entry in WIDGETS.get(widget_type).unwrap() {
                        div {
                            class: "col-4 card mx-auto",
                            onclick: move |_| state.write().current_widget = widget_entry.widget,
                            div {
                                class: "card-body stretched-link",
                                h5 {
                                    class: "card-title",
                                    widget_entry.title
                                }
                                p {
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
    NumberBaseConverter,
    Base64Encoder,
    Home,
}
