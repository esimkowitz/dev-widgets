use digest::DynDigest;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsFingerprint;
use std::fmt;

use crate::components::inputs::{SwitchInput, TextAreaForm, TextInput};
use crate::widget_entry::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Hash Generator",
    short_title: "Hash",
    description: "Generate cryptographic hashes of strings",
    path: "/hash-generator",
    function: hash_generator,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsFingerprint> = WidgetIcon {
    icon: BsFingerprint,
};

pub fn hash_generator(cx: Scope) -> Element {
    use_shared_state_provider(cx, || HashGeneratorState {
        value: "".to_string(),
        uppercase: false,
    });

    let hash_generator_state = use_shared_state::<HashGeneratorState>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "number-base-converter",
            SwitchInput {
                label: "Uppercase",
                checked: hash_generator_state.read().uppercase,
                oninput: move |is_enabled| {
                    hash_generator_state.write().uppercase = is_enabled;
                }
            }
            TextAreaForm {
                label: "Value to hash",
                value: "{hash_generator_state.read().value}",
                oninput: move |event: Event<FormData>| {
                    let value = event.value.clone();
                    hash_generator_state.write().value = value;
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
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn HashField(cx: Scope, algorithm: HashingAlgorithm) -> Element {
    let hash_generator_state = use_shared_state::<HashGeneratorState>(cx).unwrap();

    let hash_generator_state_cur = hash_generator_state.read();

    let mut hasher = select_hasher(*algorithm);

    let hashed_value = generate_hash(
        hash_generator_state_cur.value.clone(),
        &mut *hasher,
        hash_generator_state_cur.uppercase,
    );

    cx.render(rsx! {
        TextInput {
            label: "{algorithm}",
            value: "{hashed_value}",
            readonly: true,
        }
    })
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
            .map(|byte| format!("{:X}", byte))
            .collect()
    } else {
        hashed_value
            .iter()
            .map(|byte| format!("{:x}", byte))
            .collect()
    }
}

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
