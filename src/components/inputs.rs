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
    + Into<String>
    + FromStr
    + Default
    + Debug
    + Display
    + Copy
    + Clone
    + PartialEq
{
}

pub fn SelectForm<T: SelectFormEnum>(props: SelectFormProps<T>) -> Element {
    rsx! {
        div {
            class: "select-form",
            select {
                id: "{props.label}",
                aria_label: "{props.label}",
                oninput: move |event| if let Ok(value) = event.parsed::<T>() {
                    props.oninput.call(value);
                },
                for enumInst in T::iter() {
                    option {
                        value: "{enumInst.into()}",
                        selected: enumInst == props.value,
                        "{enumInst.into()}"
                    }
                }
            }
            label {
                r#for: "{props.label}",
                "{props.label}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectFormProps<T: SelectFormEnum + 'static> {
    label: String,
    value: T,
    oninput: EventHandler<T>,
}

#[component]
pub fn SwitchInput(
    label: String,
    checked: bool,
    oninput: EventHandler<bool>,
) -> Element {
    rsx! {
        div {
            class: "switch-input",
            input {
                r#type: "checkbox",
                id: "{label}",
                role: "switch",
                checked: "{checked}",
                oninput: move |event| {
                    let is_enabled = event.checked();
                    oninput.call(is_enabled);
                }
            }
            label {
                r#for: "{label}",
                "{label}"
            }
        }
    }
}

#[component]
pub fn TextAreaForm(
    class: Option<String>,
    value: String,
    label: String,
    readonly: Option<bool>,
    oninput: Option<EventHandler<Event<FormData>>>,
    onchange: Option<EventHandler<Event<FormData>>>,
) -> Element {
    let readonly = readonly.unwrap_or(false);
    let classLocal: String = class.unwrap_or_default();
    rsx! {
        div {
            class: "textarea-form {classLocal}",
            id: "{label}",
            textarea {
                value: "{value}",
                oninput:  move |event| if let Some(oninput) = oninput {
                    oninput.call(event);
                },
                onchange: move |event| if let Some(onchange) = onchange {
                    onchange.call(event);
                },
                readonly: readonly,
            }
            label {
                r#for: "{label}",
                {label.clone()}
            }
        }
    }
}

#[component]
pub fn TextInput(
    value: String,
    label: String,
    oninput: Option<EventHandler<Event<FormData>>>,
    onchange: Option<EventHandler<Event<FormData>>>,
    readonly: Option<bool>,
) -> Element {
    let readonly = readonly.unwrap_or(false);

    rsx! {
        div {
            class: "text-input",
            div {
                class: "form-floating",
                input {
                    class: "form-control",
                    r#type: "text",
                    value: "{value}",
                    oninput: move |event| {
                        if let Some(oninput) = oninput {
                            oninput.call(event);
                        }
                    },
                    onchange: move |event| {
                        if let Some(onchange) = onchange {
                            onchange.call(event);
                        }
                    },
                    readonly: readonly
                }
                label {
                    r#for: "{label}",
                    {label.clone()}
                }
            }
        }
    }
}

#[component]
pub fn NumberInput<T: PrimInt + Display + Default + FromStr + 'static>(
    class: Option<&'static str>,
    value: T,
    label: &'static str,
    onchange: EventHandler<T>,
) -> Element {
    rsx! {
        div {
            class: "number-input {class.unwrap_or_default()}",
            div {
                class: "input-group",
                div {
                    class: "input-and-label",
                    input {
                        r#type: "number",
                        value: "{value}",
                        id: "{label}",
                        onchange: move |event| if let Ok(value) = event.parsed::<T>() {
                            onchange.call(value);
                        }
                    }
                    label {
                        r#for: "{label}",
                        {label}
                    }
                }
                div {
                    class: "inc-dec-buttons",
                    button {
                        onclick: move |_| if let Some(value) = value.checked_add(&T::one()) {
                            onchange.call(value);
                        },
                        Icon {
                            icon: BsPlus,
                            class: "button-icon",
                            height: 15,
                            width: 15,
                        }
                    }
                    button {
                        onclick: move |_| if let Some(value) = value.checked_sub(&T::one()) {
                            onchange.call(value);
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
    }
}
