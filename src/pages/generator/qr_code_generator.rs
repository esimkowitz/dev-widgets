#![allow(non_snake_case)]
use base64ct::{Base64, Encoding};
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaQrcode;

use qrcode_generator::{
    qr::{Encoder, ErrorCorrection},
    Renderer,
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{
    components::inputs::{SelectForm, SelectFormEnum, TextAreaForm},
    pages::{WidgetEntry, WidgetIcon},
    storage::use_local_persistent,
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "QR Code Generator",
    short_title: "QR Code",
    description: "Generate QR codes from text",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<FaQrcode> = WidgetIcon { icon: FaQrcode };

pub fn QrCodeGenerator() -> Element {
    let mut qr_code_value = use_local_persistent("qr.value", || "".to_string());
    let mut qr_code_error_correction = use_local_persistent("qr.ecc", Ecc::default);

    let qr_code_result = Encoder::new((*qr_code_error_correction.read()).into())
        .encode_text(&*qr_code_value.read())
        .ok()
        .and_then(|symbol| Renderer::new(&symbol, 1024).to_svg_string(None::<&str>).ok())
        .map(|svg| Base64::encode_string(svg.as_bytes()))
        .unwrap_or_default();

    rsx! {
        div { class: "widget qr-code-generator",
            SelectForm::<Ecc> {
                label: "Error Correction Level",
                oninput: move |ecc: Ecc| {
                    qr_code_error_correction.set(ecc);
                },
                value: *qr_code_error_correction.read(),
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
                src: "data:image/svg+xml;base64,{qr_code_result}",
            }
        }
    }
}

#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Display,
    EnumIter,
    EnumString,
    Hash,
    IntoStaticStr,
    PartialEq,
    Serialize,
    Deserialize,
)]
enum Ecc {
    #[default]
    Low,
    Medium,
    Quartile,
    High,
}

impl SelectFormEnum for Ecc {}

impl From<Ecc> for String {
    fn from(ecc: Ecc) -> Self {
        ecc.to_string()
    }
}

impl From<Ecc> for ErrorCorrection {
    fn from(ecc: Ecc) -> Self {
        match ecc {
            Ecc::Low => ErrorCorrection::Low,
            Ecc::Medium => ErrorCorrection::Medium,
            Ecc::Quartile => ErrorCorrection::Quartile,
            Ecc::High => ErrorCorrection::High,
        }
    }
}
