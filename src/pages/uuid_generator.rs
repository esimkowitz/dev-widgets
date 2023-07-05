use dioxus::{prelude::*, html::label};
use dioxus_free_icons::icons::bs_icons::BsGlobe2;

use crate::{widget_entry::{WidgetEntry, WidgetIcon}, components::inputs::{SwitchInput, NumberInput, TextAreaForm}};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "UUID/GUID Generator",
    short_title: "UUID",
    description: "Generate unique identifiers",
    path: "/uuid-generator",
    function: uuid_generator,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsGlobe2> = WidgetIcon { icon: BsGlobe2 };

pub fn uuid_generator(cx: Scope) -> Element {
    let hyphens_state = use_state(cx, || true);
    let uppercase_state = use_state(cx, || true);
    let num_uuids_state = use_state(cx, || 1);
    let uuids_state = use_ref(cx, || vec![String::new()]);

    let uuids = uuids_state.read().join("\n");
    cx.render(rsx! {
        div {
            class: "uuid-generator",
            SwitchInput {
                label: "Hyphens",
                checked: true,
                oninput: move |value| {
                    println!("Hyphens: {}", value);
                    hyphens_state.set(value);
                }
            }
            SwitchInput {
                label: "Uppercase",
                checked: true,
                oninput: move |value| {
                    println!("Uppercase: {}", value);
                    uppercase_state.set(value);
                }
            }
            NumberInput::<usize> {
                label: "Number of UUIDs to generate",
                value: **num_uuids_state,
                onchange: move |value| {
                    println!("Number of UUIDs: {}", value);
                    num_uuids_state.set(value);
                }
            }
            TextAreaForm {
                label: "UUIDs",
                value: "{uuids}",
                readonly: true,
            }
        }
    })
}
