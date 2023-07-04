#![allow(non_snake_case)]
use std::fmt::{Debug, Display};
use std::str::FromStr;

use dioxus::prelude::*;

use strum::IntoEnumIterator;

pub struct SelectForm<'a, T: IntoEnumIterator + Into<&'static str> + FromStr + Default> {
    phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T: IntoEnumIterator + Into<&'static str> + FromStr + Default> SelectForm<'a, T> {
    pub fn SelectForm(cx: Scope<'a, SelectFormProps<'a, T>>) -> Element<'a> {
        cx.render(rsx! {
            div {
                class: "select-form",
                select {
                    id: "{cx.props.label}",
                    aria_label: "{cx.props.label}",
                    oninput: move |event| {
                        cx.props.oninput.call(T::from_str(&event.value).unwrap_or_default());
                    },
                    for enumInst in T::iter() {
                        option {
                            value: "{enumInst.into()}",
                            "{enumInst.into()}"
                        }
                    }
                }
                label {
                    r#for: "{cx.props.label}",
                    "{cx.props.label}"
                }
            }
        })
    }
}

#[derive(Props)]
struct SelectFormProps<'a, T: IntoEnumIterator + Into<&'static str> + FromStr + Default> {
    label: &'a str,
    oninput: EventHandler<'a, T>,
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
pub fn TextInput<'a>(
    cx: Scope<'a>,
    value: &'a str,
    label: &'a str,
    oninput: EventHandler<'a, Event<FormData>>,
) -> Element<'a> {
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
