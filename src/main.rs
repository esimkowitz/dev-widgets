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

    fn set_display(current_widget: &UseState<CurrentWidget>, desired_widget: CurrentWidget) -> &str {
        if *current_widget.get() == desired_widget { "block" } else { "none" }
    }

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
                display: set_display(current_widget, CurrentWidget::Base64Converter),
                base64_converter::Base64Converter {}
            }
            div {
                display: set_display(current_widget, CurrentWidget::NumberBaseConverter),
                number_base_converter::NumberBaseConverter {}
            }
        }
    })
}

#[derive(PartialEq)]
enum CurrentWidget {
    NumberBaseConverter,
    Base64Converter,
}