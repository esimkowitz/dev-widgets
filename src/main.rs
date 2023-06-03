#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*};

pub mod number_base_converter;
pub mod base64_converter;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let current_widget = use_state(cx, || CurrentWidget::NumberBaseConverter);

    cx.render(rsx! {
        div {
            div {
                button {
                    onclick: move |_| current_widget.set(CurrentWidget::NumberBaseConverter),
                    "Number base converter"
                }
                button {
                    onclick: move |_| current_widget.set(CurrentWidget::Base64Converter),
                    "Base64 converter"
                }
            }
            div {
                WidgetView {
                    current_widget: *current_widget.get()
                }
            }
        }
    })
}

fn WidgetView(cx: Scope<WidgetViewProps>) -> Element {
    fn set_display(current_widget: CurrentWidget, desired_widget: CurrentWidget) -> &'static str {
        if current_widget == desired_widget { "block" } else { "none" }
    }
    cx.render(rsx! {
        div {
            display: set_display(cx.props.current_widget, CurrentWidget::Base64Converter),
            base64_converter::Base64Converter {}
        }
        div {
            display: set_display(cx.props.current_widget, CurrentWidget::NumberBaseConverter),
            number_base_converter::NumberBaseConverter {}
        }
    })
}

#[derive(PartialEq, Props)]
struct WidgetViewProps {
    current_widget: CurrentWidget,
}

#[derive(PartialEq, Copy, Clone)]
enum CurrentWidget {
    NumberBaseConverter,
    Base64Converter,
}