#![allow(non_snake_case)]
use base64ct::{Base64, Encoding};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsQrCode;

use qrcode_generator::{self};
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{
    components::inputs::{SelectForm, SelectFormEnum, TextAreaForm},
    pages::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "QR Code Generator",
    short_title: "QR Code",
    description: "Generate QR codes from text",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<BsQrCode> = WidgetIcon { icon: BsQrCode };

pub fn QrCodeGenerator() -> Element {
    let mut qr_code_value = use_signal(|| "".to_string());
    let mut qr_code_error_correction = use_signal(Ecc::default);

    let qr_code_result = qrcode_generator::to_svg_to_string(
        (*qr_code_value.read()).clone(),
        (*qr_code_error_correction.read()).into(),
        1024,
        None::<&str>,
    );
    let qr_code_result = match qr_code_result {
        Ok(result) => Base64::encode_string(result.as_bytes()),
        Err(_) => "".to_string(),
    };

    // TODO: fix errors
    rsx! {
        div {
            class: "qr-code-generator",
            SelectForm::<Ecc> {
                label: "Error Correction Level",
                oninput: move |ecc: Ecc| {
                    qr_code_error_correction.set(ecc);
                },
                value: (*qr_code_error_correction.read()).clone(),
            }
            TextAreaForm {
                label: "Input",
                value: qr_code_value,
                oninput: move |event: Event<FormData>| {
                    qr_code_value.set(event.value());
                },
            }

            div {
                class: "alert alert-warning",
                display: if !qr_code_result.is_empty() { "none" } else { "block" },
                "Input string is too long to generate a QR code with this level of error correction."
            }
            img {
                class: "qr-code",
                display: if qr_code_result.is_empty() { "none" } else { "block" },
                src: "data:image/svg+xml;base64,{qr_code_result}"
            }
        }
    }
}

#[derive(
    Copy, Clone, Default, Debug, Display, EnumIter, EnumString, Hash, IntoStaticStr, PartialEq
)]
enum Ecc {
    #[default]
    Low,
    Medium,
    Quartile,
    High,
}

impl Into<String> for Ecc {
    fn into(self) -> String {
        self.to_string()
    }
}

impl SelectFormEnum for Ecc {}

impl From<Ecc> for qrcode_generator::QrCodeEcc {
    fn from(ecc: Ecc) -> Self {
        match ecc {
            Ecc::Low => qrcode_generator::QrCodeEcc::Low,
            Ecc::Medium => qrcode_generator::QrCodeEcc::Medium,
            Ecc::Quartile => qrcode_generator::QrCodeEcc::Quartile,
            Ecc::High => qrcode_generator::QrCodeEcc::High,
        }
    }
}
