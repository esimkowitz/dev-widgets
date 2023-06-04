// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*};
use phf::phf_map;

pub mod base64_encoder;
pub mod number_base_converter;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(app);
}

static WIDGETS: phf::Map<&str, &'static [WidgetEntry]> = phf_map! {
    "Encoder" => &[
        WidgetEntry {
            title: base64_encoder::TITLE,
            widget_type: WidgetType::Encoder,
            widget: Widget::Base64Encoder,
            function: base64_encoder::base64_encoder,
        }
    ],
    "Converter" => &[
        WidgetEntry {
            title: number_base_converter::TITLE,
            widget_type: WidgetType::Converter,
            widget: Widget::NumberBaseConverter,
            function: number_base_converter::number_base_converter,
        }
    ],
};

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || WidgetViewState {
        current_widget: Widget::Home,
    });
    let state = use_shared_state::<WidgetViewState>(cx).unwrap();

    cx.render(rsx! {
        link { rel: "stylesheet", href: "../src/style.css" },
        div {
            class: "sidenav",
            a {
                onclick: move |_| state.write().current_widget = Widget::Home,
                "Home"
            }
            for widget_type in WIDGETS.keys() {
                details {
                    summary {
                        class: "section-header",
                        *widget_type
                    }
                    for widget_entry in WIDGETS.get(widget_type).unwrap() {
                        a {
                            class: "section-item",
                            onclick: move |_| state.write().current_widget = widget_entry.widget,
                            widget_entry.title
                        }
                    }
                }
            }
        }
        div {
            class: "main",
            widget_view {}
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
            display: set_display(state.read().current_widget, Widget::Base64Encoder),
            base64_encoder::base64_encoder {}
        }
        div {
            display: set_display(state.read().current_widget, Widget::NumberBaseConverter),
            number_base_converter::number_base_converter {}
        }
        div {
            display: set_display(state.read().current_widget, Widget::Home),
            home_page {}
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

            ul {
                for widget_type in WIDGETS.keys() {
                    for widget_entry in WIDGETS.get(widget_type).unwrap() {
                        li {
                            a {
                                onclick: move |_| state.write().current_widget = widget_entry.widget,
                                widget_entry.title
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
    widget_type: WidgetType,
    widget: Widget,
    function: fn(Scope) -> Element,
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
