
use dioxus::{prelude::*};
use base64::{Engine as _, engine::{general_purpose}};

pub fn Base64Converter(cx: Scope) -> Element {
    use_shared_state_provider(cx, || ConverterValue { encoded_value: String::new(), decoded_value: String::new() });
    cx.render(rsx! {
        div {
            h2 {
                "Base64 converter"
            }
            div {
                ConverterInput {
                    direction: Direction::Encode
                }
                ConverterInput {
                    direction: Direction::Decode
                }
            }
        }
    })
}

fn ConverterInput(cx: Scope<ConverterInputProps>) -> Element {
    let value_context = use_shared_state::<ConverterValue>(cx).unwrap();
    let display_value = match cx.props.direction {
        Direction::Encode => value_context.read().decoded_value.clone(),
        Direction::Decode => value_context.read().encoded_value.clone(),
    };

    const NOT_STRING: &str = "Not String";
    cx.render(rsx! {
        div {
            span {
                match cx.props.direction {
                    Direction::Encode => "Encode",
                    Direction::Decode => "Decode",
                }
            }
            input {
                value: "{display_value}",
                oninput: move |event| {
                    let input_value = event.value.clone();
                    match cx.props.direction {
                        Direction::Encode => {
                            value_context.write().decoded_value = input_value.clone();
                            value_context.write().encoded_value = general_purpose::STANDARD.encode(input_value);
                        },
                        Direction::Decode => {
                            value_context.write().encoded_value = input_value.clone();
                            let decode_val = general_purpose::STANDARD.decode(input_value).unwrap_or_default();
                            value_context.write().decoded_value = String::from_utf8(decode_val).unwrap_or(NOT_STRING.to_string());
                        },
                    };
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct ConverterInputProps {
    direction: Direction
}

struct ConverterValue{
    encoded_value: String,
    decoded_value: String,
}

#[derive(PartialEq)]
enum Direction {
    Encode,
    Decode,
}