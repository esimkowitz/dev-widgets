use base64::{engine::general_purpose, Engine as _};
use dioxus::{prelude::*};
use std::fmt;

pub const TITLE: &str = "Base64 Encoder / Decoder";
pub const DESCRIPTION: &str = "Encode and decode base64 strings";

pub fn base64_encoder(cx: Scope) -> Element {
    use_shared_state_provider(cx, || EncoderValue {
        encoded_value: String::new(),
        decoded_value: String::new(),
    });
    cx.render(rsx! {
        div {
            class: "base64-encoder",
            div {
                class: "widget-title",
                TITLE
            }
            div {
                class: "widget-body",
                encoder_input {
                    direction: Direction::Encode
                }
                encoder_input {
                    direction: Direction::Decode
                }
            }
        }
    })
}

#[inline_props]
fn encoder_input(cx: Scope, direction: Direction) -> Element {
    let value_context = use_shared_state::<EncoderValue>(cx).unwrap();
    let display_value = match direction {
        Direction::Encode => value_context.read().decoded_value.clone(),
        Direction::Decode => value_context.read().encoded_value.clone(),
    };

    const NOT_STRING: &str = "Not String";
    cx.render(rsx! {
        div {
            class: "form-floating mb-3",
            style: "height: 14em;",
            textarea {
                class: "form-control",
                value: "{display_value}",
                id: "{direction}",
                oninput: move |event| {
                    let input_value = event.value.clone();
                    match direction {
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
            label {
                "for": "{direction}",
                match direction {
                    Direction::Encode => "Encode",
                    Direction::Decode => "Decode",
                }
            }
        }
    })
}

struct EncoderValue {
    encoded_value: String,
    decoded_value: String,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Encode,
    Decode,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}