#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Accordion(children: Element, title: String, is_open: Option<bool>) -> Element {
    let default_open_flag = !is_open.unwrap_or(false);
    let mut is_close_accordion = use_signal(|| default_open_flag);
    let buttoncss = if *is_close_accordion.read() {
        "accordion-button p-2 collapsed"
    } else {
        "accordion-button p-2"
    };
    let accordioncss = if *is_close_accordion.read() {
        "accordion-collapse collapse"
    } else {
        "accordion-collapse collapse show"
    };
    rsx! {
        div { class: "accordion-item",
            h3 { class: "accordion-header",
                button {
                    class: "{buttoncss}",
                    r#type: "button",
                    aria_expanded: "{!*is_close_accordion.read()}",
                    onclick: move |_| {
                        is_close_accordion.with_mut(|flag| *flag = !*flag);
                    },
                    {title}
                }
            }
            div { class: "{accordioncss}",
                div { class: "accordion-body p-0", {children} }
            }
        }
    }
}
