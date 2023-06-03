
use dioxus::{prelude::*};

pub fn NumberBaseConverter(cx: Scope) -> Element {
    use_shared_state_provider(cx, || ConverterValue(0));
    cx.render(rsx! {
        div {
            h2 {
                "Number base converter"
            }
            div {
                ConverterInput {
                    base: NumberBase::Decimal
                }
                ConverterInput {
                    base: NumberBase::Hexadecimal
                }
                ConverterInput {
                    base: NumberBase::Octal
                }
                ConverterInput {
                    base: NumberBase::Binary
                }
            }
        }
    })
}

fn ConverterInput(cx: Scope<ConverterInputProps>) -> Element {
    let value_context = use_shared_state::<ConverterValue>(cx).unwrap();
    
    let current_value = value_context.read().0;
    let formatted_value = match cx.props.base {
        NumberBase::Binary => format!("{:b}", current_value),
        NumberBase::Octal => format!("{:o}", current_value),
        NumberBase::Decimal => format!("{}", current_value),
        NumberBase::Hexadecimal => format!("{:x}", current_value),
    };
    cx.render(rsx! {
        div {
            span {
                match cx.props.base {
                    NumberBase::Binary => "Binary",
                    NumberBase::Octal => "Octal",
                    NumberBase::Decimal => "Decimal",
                    NumberBase::Hexadecimal => "Hexadecimal",
                }
            }
            input {
                value: "{formatted_value}",
                oninput: move |event| {
                    let event_value = event.value.clone();
                    value_context.write().0 = match cx.props.base {
                        NumberBase::Binary => i64::from_str_radix(&event_value, 2),
                        NumberBase::Octal => i64::from_str_radix(&event_value, 8),
                        NumberBase::Decimal => i64::from_str_radix(&event_value, 10),
                        NumberBase::Hexadecimal => i64::from_str_radix(&event_value, 16),
                    }.unwrap_or(0);
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct ConverterInputProps {
    base: NumberBase
}

struct ConverterValue(i64);

#[derive(PartialEq)]
enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal
}