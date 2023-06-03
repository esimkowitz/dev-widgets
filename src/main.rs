// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

pub mod base64_converter;
pub mod number_base_converter;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
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
                widget_view {
                    current_widget: *current_widget.get()
                }
            }
        }
    })
}

#[inline_props]
fn widget_view(cx: Scope, current_widget: CurrentWidget) -> Element {
    fn set_display(current_widget: CurrentWidget, desired_widget: CurrentWidget) -> &'static str {
        if current_widget == desired_widget {
            "block"
        } else {
            "none"
        }
    }
    cx.render(rsx! {
        div {
            display: set_display(*current_widget, CurrentWidget::Base64Converter),
            base64_converter::base64_converter {}
        }
        div {
            display: set_display(*current_widget, CurrentWidget::NumberBaseConverter),
            number_base_converter::number_base_converter {}
        }
    })
}

#[derive(PartialEq, Copy, Clone)]
enum CurrentWidget {
    NumberBaseConverter,
    Base64Converter,
}
