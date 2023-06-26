#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn TextAreaForm<'a>(
    cx: Scope<'a>,
    value: &'a str,
    label: &'a str,
    oninput: EventHandler<'a, Event<FormData>>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "textarea-form",
            id: "{label}",
            textarea {
                value: "{value}",
                oninput: move |event| oninput.call(event)
            }
            label {
                r#for: "{label}",
                *label
            }
        }
    })
}
