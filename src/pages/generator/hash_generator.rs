#![allow(non_snake_case)]
use digest::DynDigest;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsFingerprint;
use std::fmt::{self, Write};

use crate::components::inputs::{SwitchInput, TextAreaForm, TextInput};
use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Hash Generator",
    short_title: "Hash",
    description: "Generate cryptographic hashes of strings",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<BsFingerprint> = WidgetIcon {
    icon: BsFingerprint,
};

pub fn HashGenerator() -> Element {
    let mut hash_generator_state = use_context_provider(|| HashGeneratorState {
        value: "".to_string(),
        uppercase: false,
    });

    rsx! {
        div {
            class: "number-base-converter",
            SwitchInput {
                label: "Uppercase",
                checked: hash_generator_state.uppercase,
                oninput: move |is_enabled| {
                    hash_generator_state.uppercase = is_enabled;
                }
            }
            TextAreaForm {
                label: "Value to hash",
                value: "{hash_generator_state.value}",
                oninput: move |event: Event<FormData>| {
                    hash_generator_state.value = event.value();
                }
            }
            HashField {
                algorithm: HashingAlgorithm::MD5,
            }
            HashField {
                algorithm: HashingAlgorithm::SHA1,
            }
            HashField {
                algorithm: HashingAlgorithm::SHA256,
            }
            HashField {
                algorithm: HashingAlgorithm::SHA512,
            }
        }
    }
}

#[component]
fn HashField(algorithm: HashingAlgorithm) -> Element {
    let hash_generator_state = use_context::<HashGeneratorState>();

    let mut hasher = select_hasher(algorithm);

    let hashed_value = generate_hash(
        hash_generator_state.value.clone(),
        &mut *hasher,
        hash_generator_state.uppercase,
    );

    rsx! {
        TextInput {
            label: "{algorithm}",
            value: "{hashed_value}",
            readonly: true,
        }
    }
}

fn select_hasher(algorithm: HashingAlgorithm) -> Box<dyn DynDigest> {
    match algorithm {
        HashingAlgorithm::MD5 => Box::<md5::Md5>::default(),
        HashingAlgorithm::SHA1 => Box::<sha1::Sha1>::default(),
        HashingAlgorithm::SHA256 => Box::<sha2::Sha256>::default(),
        HashingAlgorithm::SHA512 => Box::<sha2::Sha512>::default(),
    }
}

fn generate_hash(value: String, hasher: &mut dyn DynDigest, uppercase: bool) -> String {
    hasher.update(value.as_bytes());
    let hashed_value = hasher.finalize_reset();

    if uppercase {
        hashed_value
            .iter()
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "{:X}", b);
                output
            })
    } else {
        hashed_value
            .iter()
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "{:x}", b);
                output
            })
    }
}

#[derive(Clone)]
struct HashGeneratorState {
    value: String,
    uppercase: bool,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum HashingAlgorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
}

impl fmt::Display for HashingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
