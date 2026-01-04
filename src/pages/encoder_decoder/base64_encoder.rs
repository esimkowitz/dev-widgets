#![allow(non_snake_case)]
use base64ct::{Base64, Encoding};
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaHashtag;
use std::fmt;

use crate::components::inputs::TextAreaForm;
use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Base64 Encoder / Decoder",
    short_title: "Base64",
    description: "Encode and decode base64 strings",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<FaHashtag> = WidgetIcon { icon: FaHashtag };

pub fn Base64Encoder() -> Element {
    use_context_provider(|| {
        Signal::new(EncoderValue {
            encoded_value: String::new(),
            decoded_value: String::new(),
        })
    });
    rsx! {
        div { class: "widget",
            encoder_input { direction: Direction::Encode }
            encoder_input { direction: Direction::Decode }
        }
    }
}

#[allow(unused_assignments, unused_variables)]
#[component]
fn encoder_input(direction: Direction) -> Element {
    let mut value_context = use_context::<Signal<EncoderValue>>();

    let current_value = value_context.with(|value| match direction {
        Direction::Encode => value.decoded_value.clone(),
        Direction::Decode => value.encoded_value.clone(),
    });

    const NOT_STRING: &str = "Not String";
    rsx! {
        TextAreaForm {
            label: match direction {
                Direction::Encode => "Text",
                Direction::Decode => "Encoded",
            },
            value: "{current_value}",
            oninput: move |event: Event<FormData>| {
                let input_value = event.value();
                match direction {
                    Direction::Encode => {
                        value_context
                            .set(EncoderValue {
                                encoded_value: Base64::encode_string(input_value.as_bytes()),
                                decoded_value: input_value,
                            });
                    }
                    Direction::Decode => {
                        let decode_val = match Base64::decode_vec(input_value.as_str()) {
                            Ok(val) => String::from_utf8(val).unwrap_or(NOT_STRING.to_string()),
                            Err(_) => NOT_STRING.to_string(),
                        };
                        value_context
                            .set(EncoderValue {
                                encoded_value: input_value,
                                decoded_value: decode_val,
                            });
                    }
                };
            },
        }
    }
}

#[derive(Clone)]
struct EncoderValue {
    encoded_value: String,
    decoded_value: String,
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Encode,
    Decode,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
