use dioxus::prelude::*;

pub const TITLE: &str = "Number Base Converter";

pub fn number_base_converter(cx: Scope) -> Element {
    use_shared_state_provider(cx, || ConverterValue(0));
    cx.render(rsx! {
        div {
            h2 {
                TITLE
            }
            div {
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
    })
}

#[inline_props]
fn converter_input(cx: Scope, base: NumberBase) -> Element {
    let value_context = use_shared_state::<ConverterValue>(cx).unwrap();

    let current_value = value_context.read().0;
    let formatted_value = match base {
        NumberBase::Binary => format!("{:b}", current_value),
        NumberBase::Octal => format!("{:o}", current_value),
        NumberBase::Decimal => format!("{}", current_value),
        NumberBase::Hexadecimal => format!("{:X}", current_value),
    };
    cx.render(rsx! {
        div {
            span {
                match base {
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
                    value_context.write().0 = match base {
                        NumberBase::Binary => i64::from_str_radix(&event_value, 2),
                        NumberBase::Octal => i64::from_str_radix(&event_value, 8),
                        NumberBase::Decimal => event_value.parse::<i64>(),
                        NumberBase::Hexadecimal => i64::from_str_radix(&event_value, 16),
                    }.unwrap_or(0);
                }
            }
        }
    })
}

struct ConverterValue(i64);

#[derive(PartialEq)]
enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}
