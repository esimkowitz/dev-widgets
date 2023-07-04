use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsQrCode;

use qrcode_generator;
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{
    components::inputs::{SelectForm, SelectFormEnum, TextAreaForm},
    widget_entry::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "QR Code Generator",
    short_title: "QR Code",
    description: "Generate QR codes from text",
    path: "/qr-code-generator",
    function: qr_code_generator,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsQrCode> = WidgetIcon { icon: BsQrCode };

pub fn qr_code_generator(cx: Scope) -> Element {
    let qr_code_value = use_state(cx, || "".to_string());
    let qr_code_error_correction = use_state(cx, || Ecc::default());

    let result = qrcode_generator::to_svg_to_string(
        qr_code_value.get(),
        qrcode_generator::QrCodeEcc::from(qr_code_error_correction.get()),
        1024,
        None::<&str>,
    );
    let result = match result {
        Ok(result) => result,
        Err(_) => "".to_string(),
    };
    let result = general_purpose::STANDARD.encode(result);

    cx.render(rsx! {
        div {
            class: "qr-code-generator",
            SelectForm::<Ecc> {
                label: "Error Correction Level",
                oninput: |ecc: Ecc| {
                    qr_code_error_correction.set(ecc);
                }
            }
            TextAreaForm {
                label: "Input",
                value: qr_code_value,
                oninput: |event: Event<FormData>| {
                    qr_code_value.set(event.value.clone());
                }
            }

            div {
                class: "alert alert-warning",
                display: if !result.is_empty() { "none" } else { "block" },
                "Input string is too long to generate a QR code with this level of error correction."
            }
            img {
                class: "qr-code",
                display: if result.is_empty() { "none" } else { "block" },
                src: "data:image/svg+xml;base64,{result}"
            }
        }
    })
}

#[derive(Copy, Clone, Default, Debug, Display, EnumIter, EnumString, Hash, IntoStaticStr)]
enum Ecc {
    #[default]
    Low,
    Medium,
    Quartile,
    High,
}

impl SelectFormEnum for Ecc {}

impl From<&Ecc> for qrcode_generator::QrCodeEcc {
    fn from(ecc: &Ecc) -> Self {
        match *ecc {
            Ecc::Low => qrcode_generator::QrCodeEcc::Low,
            Ecc::Medium => qrcode_generator::QrCodeEcc::Medium,
            Ecc::Quartile => qrcode_generator::QrCodeEcc::Quartile,
            Ecc::High => qrcode_generator::QrCodeEcc::High,
        }
    }
}
