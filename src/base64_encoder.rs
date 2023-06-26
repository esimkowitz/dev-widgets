use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsHash;
use std::fmt;

use crate::widget_entry::{WidgetEntry, WidgetIcon};
use crate::textarea_form::TextAreaForm;

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Base64 Encoder / Decoder",
    short_title: "Base64",
    description: "Encode and decode base64 strings",
    path: "/base64-encoder",
    function: base64_encoder,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsHash> = WidgetIcon {
    icon: BsHash,
};

pub fn base64_encoder(cx: Scope) -> Element {
    use_shared_state_provider(cx, || EncoderValue {
        encoded_value: String::new(),
        decoded_value: String::new(),
    });
    cx.render(rsx! {
        div {
            class: "base64-encoder",
            encoder_input {
                direction: Direction::Encode
            }
            encoder_input {
                direction: Direction::Decode
            }
        }
    })
}

#[inline_props]
fn encoder_input(cx: Scope, direction: Direction) -> Element {
    let value_context = use_shared_state::<EncoderValue>(cx).unwrap();

    let current_value = match direction {
        Direction::Encode => value_context.read().decoded_value.clone(),
        Direction::Decode => value_context.read().encoded_value.clone(),
    };

    const NOT_STRING: &str = "Not String";
    cx.render(rsx! {
        TextAreaForm {
            label: match direction {
                Direction::Encode => "Encode",
                Direction::Decode => "Decode",
            },
            value: "{current_value}",
            oninput: move |event: Event<FormData>| {
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
