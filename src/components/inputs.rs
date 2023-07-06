#![allow(non_snake_case)]
use std::fmt::{Debug, Display};
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsDash, BsPlus},
    Icon,
};
use num_traits::PrimInt;
use strum::IntoEnumIterator;

pub trait SelectFormEnum:
    IntoEnumIterator
    + Into<&'static str>
    + FromStr
    + Default
    + Debug
    + Display
    + Copy
    + Clone
    + PartialEq
{
}

pub fn SelectForm<'a, T: SelectFormEnum>(cx: Scope<'a, SelectFormProps<'a, T>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "select-form",
            select {
                id: "{cx.props.label}",
                aria_label: "{cx.props.label}",
                oninput: move |event| {
                    if let Ok(value) = T::from_str(&event.value) {
                        cx.props.oninput.call(value);
                    }
                },
                for enumInst in T::iter() {
                    option {
                        value: "{enumInst.into()}",
                        selected: enumInst == cx.props.value,
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

#[derive(Props)]
pub struct SelectFormProps<'a, T: SelectFormEnum> {
    label: &'a str,
    value: T,
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
    readonly: Option<bool>,
    oninput: Option<EventHandler<'a, Event<FormData>>>,
    onchange: Option<EventHandler<'a, Event<FormData>>>,
) -> Element<'a> {
    let readonly = readonly.unwrap_or(false);
    cx.render(rsx! {
        div {
            class: "textarea-form",
            id: "{label}",
            textarea {
                value: "{value}",
                oninput:  move |event| match oninput {
                    Some(oninput) => oninput.call(event),
                    None => {}
                },
                onchange: move |event| match onchange {
                    Some(onchange) => onchange.call(event),
                    None => {}
                },
                readonly: readonly,
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
    oninput: Option<EventHandler<'a, Event<FormData>>>,
    onchange: Option<EventHandler<'a, Event<FormData>>>,
    readonly: Option<bool>,
) -> Element<'a> {
    let readonly = readonly.unwrap_or(false);
    cx.render(rsx! {
        div {
            class: "text-input",
            input {
                r#type: "text",
                value: "{value}",
                oninput: move |event| match oninput {
                    Some(oninput) => oninput.call(event),
                    None => {}
                },
                onchange: move |event| match onchange {
                    Some(onchange) => onchange.call(event),
                    None => {}
                },
                readonly: readonly
            }
            label {
                r#for: "{label}",
                *label
            }
        }
    })
}

#[inline_props]
pub fn NumberInput<'a, T: PrimInt + Display + Default + FromStr>(
    cx: Scope<'a>,
    class: Option<&'a str>,
    value: T,
    label: &'a str,
    onchange: EventHandler<'a, T>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "number-input {class.unwrap_or_default()}",
            div {
                class: "input-group",
                div {
                    class: "input-and-label",
                    input {
                        class: "form-control",
                        r#type: "number",
                        value: "{value}",
                        id: "{label}",
                        onchange: move |event| {
                            if let Ok(value) = event.value.parse::<T>() {
                                onchange.call(value);
                            }
                        }
                    }
                    label {
                        r#for: "{label}",
                        *label
                    }
                }
                div {
                    class: "btn-group-vertical input-group-text p-1",
                    button {
                        onclick: move |_| {
                            if let Some(value) = value.checked_add(&T::one()) {
                                onchange.call(value);
                            };
                        },
                        Icon {
                            icon: BsPlus,
                            class: "button-icon",
                            height: 15,
                            width: 15,
                        }
                    }
                    button {
                        onclick: move |_| {
                            if let Some(value) = value.checked_sub(&T::one()) {
                                onchange.call(value);
                            };
                        },
                        Icon {
                            icon: BsDash,
                            class: "button-icon",
                            height: 15,
                            width: 15,
                        }
                    }
                }
            }
        }
    })
}
