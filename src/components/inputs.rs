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
            class: "select-form",
            select {
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

#[inline_props]
pub fn SwitchInput<'a>(
    cx: Scope<'a>,
    label: &'a str,
    checked: bool,
    oninput: EventHandler<'a, bool>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "switch-input",
            input {
                r#type: "checkbox",
                id: "{label}",
                role: "switch",
                checked: "{checked}",
                oninput: move |event| {
                    let is_enabled = event.value == "true";
                    oninput.call(is_enabled);
                }
            }
            label {
                r#for: "{label}",
                "{label}"
            }
        }
    })
}

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

#[inline_props]
pub fn TextInput<'a>(cx: Scope<'a>, value: &'a str, label: &'a str, oninput: EventHandler<'a, Event<FormData>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "text-input",
            input {
                r#type: "text",
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