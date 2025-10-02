#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsGlobe2;
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{
    components::inputs::{NumberInput, SelectForm, SelectFormEnum, SwitchInput, TextAreaForm},
    pages::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "UUID/GUID Generator",
    short_title: "UUID",
    description: "Generate unique identifiers",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<BsGlobe2> = WidgetIcon { icon: BsGlobe2 };

pub fn UuidGenerator() -> Element {
    let mut hyphens_state = use_signal(|| true);
    let mut uppercase_state = use_signal(|| true);
    let mut num_uuids_state = use_signal(|| 1);
    let mut uuids_state = use_signal(Vec::<String>::new);
    let mut uuid_version_state = use_signal(|| UUIDVersion::V4);

    let uuids_str = uuids_state.with(|uuids_vec| uuids_vec.join("\n"));
    rsx! {
        div {
            class: "uuid-generator",
            div {
                class: "params",
                div {
                    class: "switches",
                    SwitchInput {
                        label: "Hyphens",
                        checked: true,
                        oninput: move |value| {
                            hyphens_state.set(value);
                        }
                    }
                    SwitchInput {
                        label: "Uppercase",
                        checked: true,
                        oninput: move |value| {
                            uppercase_state.set(value);
                        }
                    }
                }
                SelectForm::<UUIDVersion> {
                    label: "UUID Version",
                    value: *uuid_version_state.read(),
                    oninput: move |uuid_version| {
                        uuid_version_state.set(uuid_version);
                    }
                }
                NumberInput::<usize> {
                    label: "Number of UUIDs to generate",
                    value: *num_uuids_state.read(),
                    onchange: move |value| {
                        num_uuids_state.set(value);
                    }
                }
            }

            div {
                class: "buttons",
                button {
                    class: "btn btn-primary me-3",
                    onclick: move |_| {
                        let mut uuids = vec![];
                        for _ in 0..*num_uuids_state.read() {
                            let uuid = uuid::Uuid::new_v4();
                            let mut uuid = if *hyphens_state.read() {
                                uuid.hyphenated().to_string()
                            } else {
                                uuid.simple().to_string()
                            };
                            if *uppercase_state.read() {
                                uuid = uuid.to_uppercase();
                            }
                            uuids.push(uuid);
                        }
                        uuids_state.write().append(&mut uuids);
                    },
                    "Generate"
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| {
                        uuids_state.write().clear();
                    },
                    "Clear"
                }
            }
            TextAreaForm {
                label: "UUIDs",
                value: "{uuids_str}",
                readonly: true,
            }
        }
    }
}

#[derive(
    Copy, Clone, Default, Debug, Display, EnumIter, EnumString, Hash, IntoStaticStr, PartialEq,
)]
#[allow(clippy::upper_case_acronyms)]
enum UUIDVersion {
    #[default]
    V4,
    V7,
}

impl SelectFormEnum for UUIDVersion {}

impl From<UUIDVersion> for String {
    fn from(uuid_version: UUIDVersion) -> Self {
        uuid_version.to_string()
    }
}
