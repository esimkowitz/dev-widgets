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
    class: Option<&'a str>,
    value: &'a str,
    label: &'a str,
    readonly: Option<bool>,
    oninput: Option<EventHandler<'a, Event<FormData>>>,
    onchange: Option<EventHandler<'a, Event<FormData>>>,
) -> Element<'a> {
    let readonly = readonly.unwrap_or(false);
    cx.render(rsx! {
        div {
            class: "textarea-form {class.unwrap_or_default()}",
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
    onsubmit: Option<EventHandler<'a, String>>,
    readonly: Option<bool>,
) -> Element<'a> {
    let readonly = readonly.unwrap_or(false);

    let form_state = use_ref(cx, || value.to_string());

    let input_group_css = if onsubmit.is_some() {
        "input-group"
    } else {
        ""
    };

    let set_value = |value: String| {
        if onsubmit.is_some() {
            form_state.with_mut(|form_value| {
                *form_value = value.clone();
            });
        }
    };

    cx.render(rsx! {
        div {
            class: "text-input {input_group_css}",
            div {
                class: "form-floating",
                input {
                    class: "form-control",
                    r#type: "text",
                    value: "{value}",
                    oninput: move |event| match oninput {
                        Some(oninput) => {
                            set_value(event.value.clone());
                            oninput.call(event)
                        },
                        None => {}
                    },
                    onchange: move |event| {
                        set_value(event.value.clone());
                        match onchange {
                            Some(onchange) => onchange.call(event),
                            None => {}
                        }
                    },
                    readonly: readonly
                }
                label {
                    r#for: "{label}",
                    *label
                }
            }
            if let Some(onsubmit) = onsubmit {
                rsx! { 
                    button {
                        class: "btn btn-primary",
                        r#type: "submit",
                        onclick: move |_| {
                            let mut value = String::default();
                            form_state.with(|form_value| {
                                value = form_value.clone();
                            });
                            onsubmit.call(value);
                        },
                        "Submit"
                    }
                }
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
                    class: "inc-dec-buttons",
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
