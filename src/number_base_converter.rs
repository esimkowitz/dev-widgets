use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsHash;
use std::fmt;

use crate::{widget_entry::WidgetEntry, sidebar_icon::sidebar_icon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Number Base Converter",
    short_title: "Number Base",
    description: "Convert numbers between binary, octal, decimal, and hexadecimal",
    path: "/number-base-converter",
    function: number_base_converter,
    icon: sidebar_icon::<BsHash>,
};

pub fn number_base_converter(cx: Scope) -> Element {
    use_shared_state_provider(&cx, || ConverterValue(0));
    use_shared_state_provider(&cx, || FormatNumberState(false));

    cx.render(rsx! {
        div {
            class: "number-base-converter",
            format_number_toggle {}
            converter_input {
                base: NumberBase::Decimal
            }
            converter_input {
                base: NumberBase::Hexadecimal
            }
            converter_input {
                base: NumberBase::Octal
            }
            converter_input {
                base: NumberBase::Binary
            }
        }
    })
}

fn format_number_toggle(cx: Scope) -> Element {
    let format_number_state = use_shared_state::<FormatNumberState>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: "form-check form-switch",
            input {
                class: "form-check-input",
                r#type: "checkbox",
                id: "format-string-toggle",
                role: "switch",
                checked: "{format_number_state.read().0}",
                oninput: move |event| {
                    let is_enabled = event.value == "true";
                    format_number_state.write().0 = is_enabled;
                }
            }
            label {
                class: "form-check-label",
                r#for: "format-string-toggle",
                "Format Numbers"
            }
        }
    })
}

#[inline_props]
fn converter_input(cx: Scope, base: NumberBase) -> Element {
    let value_context = use_shared_state::<ConverterValue>(cx).unwrap();
    let format_number_state = use_shared_state::<FormatNumberState>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "form-floating mb-3",
            input {
                class: "form-control",
                value: "{format_number(value_context.read().0, *base, format_number_state.read().0)}",
                id: "{base}",
                oninput: move |event| {
                    let event_value = event.value.clone();
                    let event_value = sanitize_string(event_value);
                    value_context.write().0 = match base {
                        NumberBase::Binary => i64::from_str_radix(&event_value, 2),
                        NumberBase::Octal => i64::from_str_radix(&event_value, 8),
                        NumberBase::Decimal => event_value.parse::<i64>(),
                        NumberBase::Hexadecimal => i64::from_str_radix(&event_value, 16),
                    }.unwrap_or(0);
                }
            }
            label {
                r#for: "{base}",
                match base {
                    NumberBase::Binary => "Binary",
                    NumberBase::Octal => "Octal",
                    NumberBase::Decimal => "Decimal",
                    NumberBase::Hexadecimal => "Hexadecimal",
                }
            }
        }
    })
}

fn format_number(number: i64, base: NumberBase, format_number: bool) -> String {
    match base {
        NumberBase::Binary => {
            let number_binary = format!("{:b}", number);
            match format_number {
                true => add_number_delimiters(number_binary, ' ', 4),
                false => number_binary,
            }
        }
        NumberBase::Octal => {
            let number_octal = format!("{:o}", number);
            match format_number {
                true => add_number_delimiters(number_octal, ' ', 3),
                false => number_octal,
            }
        }
        NumberBase::Decimal => {
            let number_decimal = format!("{}", number);
            match format_number {
                true => add_number_delimiters(number_decimal, ',', 3),
                false => number_decimal,
            }
        }
        NumberBase::Hexadecimal => {
            let number_hexadecimal = format!("{:X}", number);
            match format_number {
                true => add_number_delimiters(number_hexadecimal, ' ', 4),
                false => number_hexadecimal,
            }
        }
    }
}

fn add_number_delimiters(number_str: String, delimiter: char, frequency: usize) -> String {
    number_str
        .chars()
        .rev()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % frequency == 0 {
                Some(delimiter)
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
}

fn sanitize_string(string: String) -> String {
    string
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>()
}

struct ConverterValue(i64);

struct FormatNumberState(bool);

#[derive(PartialEq, Debug, Clone, Copy)]
enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl fmt::Display for NumberBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
