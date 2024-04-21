#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::Bs123;
use std::fmt;

use crate::components::inputs::{SwitchInput, TextInput};
use crate::pages::{WidgetEntry, WidgetIcon};
use crate::utils::{add_number_delimiters, sanitize_string};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Number Base Converter",
    short_title: "Number Base",
    description: "Convert numbers between binary, octal, decimal, and hexadecimal",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<Bs123> = WidgetIcon { icon: Bs123 };

pub fn NumberBaseConverter() -> Element {
    use_context_provider(|| ConverterValue(0));
    let format_number_state = use_context_provider(|| FormatNumberState(false));

    rsx! {
        div {
            class: "number-base-converter",
            SwitchInput {
                label: "Format Numbers",
                checked: format_number_state.0,
                oninput: move |is_enabled| {
                    format_number_state.0 = is_enabled;
                }
            }
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
    }
}

#[component]
fn converter_input(base: NumberBase) -> Element {
    let value_context = use_context::<ConverterValue>();
    let format_number_state = use_context::<FormatNumberState>();

    rsx! {
        TextInput {
            label: "{base}",
            value: "{format_number(value_context.0, base, format_number_state.0)}",
            oninput: move |event: Event<FormData>| {
                let event_value = sanitize_string(event.value());
                value_context.0 = match base {
                    NumberBase::Binary => i64::from_str_radix(&event_value, 2),
                    NumberBase::Octal => i64::from_str_radix(&event_value, 8),
                    NumberBase::Decimal => event_value.parse::<i64>(),
                    NumberBase::Hexadecimal => i64::from_str_radix(&event_value, 16),
                }.unwrap_or(0);
            }
        }
    }
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


#[derive(Clone)]
struct ConverterValue(i64);

#[derive(Clone)]
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
