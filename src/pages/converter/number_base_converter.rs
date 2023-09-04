#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::Bs123;
use std::fmt;

use crate::components::inputs::{SwitchInput, TextInput};
use crate::pages::{WidgetEntry, WidgetIcon};
use crate::persistence::use_persistent;
use crate::utils::{add_number_delimiters, sanitize_string};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Number Base Converter",
    short_title: "Number Base",
    description: "Convert numbers between binary, octal, decimal, and hexadecimal",
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<Bs123> = WidgetIcon { icon: Bs123 };

pub fn NumberBaseConverter(cx: Scope) -> Element {
    let format_number_state = use_persistent(cx, "format-number-state", || false);

    render! {
        div {
            class: "number-base-converter",
            SwitchInput {
                label: "Format Numbers",
                checked: format_number_state.get(),
                oninput: move |is_enabled| {
                    format_number_state.set(is_enabled);
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

#[inline_props]
fn converter_input(cx: Scope, base: NumberBase) -> Element {
    let value_context = use_persistent(cx, "converter-value", || 0);
    let format_number_state = use_persistent(cx, "format-number-state", || false);

    render! {
        TextInput {
            label: "{base}",
            value: "{format_number(value_context.get(), *base, format_number_state.get())}",
            oninput: move |event: Event<FormData>| {
                let event_value = event.value.clone();
                let event_value = sanitize_string(event_value);
                value_context.set(match base {
                    NumberBase::Binary => i64::from_str_radix(&event_value, 2),
                    NumberBase::Octal => i64::from_str_radix(&event_value, 8),
                    NumberBase::Decimal => event_value.parse::<i64>(),
                    NumberBase::Hexadecimal => i64::from_str_radix(&event_value, 16),
                }.unwrap_or(0));
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
