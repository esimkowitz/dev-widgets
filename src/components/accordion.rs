#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn Accordion<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    title: &'a str,
    is_open: Option<bool>,
) -> Element {
    let default_open_flag = !is_open.unwrap_or(false);
    let is_close_accordion = use_state(cx, || default_open_flag);
    let buttoncss = if *is_close_accordion.get() {
        "accordion-button p-2 collapsed"
    } else {
        "accordion-button p-2"
    };
    let accordioncss = if *is_close_accordion.get() {
        "accordion-collapse collapse"
    } else {
        "accordion-collapse collapse show"
    };
    cx.render(rsx! {
        div {
            class: "accordion-item",
            h3 {
                class: "accordion-header",
                button {
                    class: "{buttoncss}",
                    r#type: "button",
                    aria_expanded: "{!is_close_accordion.get()}",
                    onclick: move |_| {
                        is_close_accordion.set(!*is_close_accordion.get());
                    },
                    *title
                }
            }
            div {
                class: "{accordioncss}",
                div {
                    class: "accordion-body p-0",
                    children
                }
            }
        }
    })
}
