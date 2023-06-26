#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn SelectForm<'a>(
    cx: Scope<'a>,
    label: &'a str,
    options: Vec<&'a str>,
    oninput: EventHandler<'a, Event<FormData>>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "select-form form-floating",
            select {
                class: "form-select",
                id: "{label}",
                aria_label: "{label}",
                oninput: move |event| oninput.call(event),
                for option in options.iter() {
                    option {
                        value: "{option}",
                        *option
                    }
                }
            }
            label {
                r#for: "{label}",
                *label
            }
        }
    })
}
