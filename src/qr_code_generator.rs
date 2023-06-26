use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsQrCode;

use qrcode_generator::QrCodeEcc;

use crate::{
    select_form::SelectForm,
    textarea_form::TextAreaForm,
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
    let qr_code_error_correction = use_state(cx, || QrCodeEcc::Low);

    let result = qrcode_generator::to_svg_to_string(
        qr_code_value.get(),
        *qr_code_error_correction.get(),
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
            SelectForm {
                label: "Error Correction Level",
                options: vec!["Low", "Medium", "Quartile", "High"],
                oninput: |event: Event<FormData>| {
                    qr_code_error_correction.set(match event.value.as_str() {
                        "Low" => QrCodeEcc::Low,
                        "Medium" => QrCodeEcc::Medium,
                        "Quartile" => QrCodeEcc::Quartile,
                        "High" => QrCodeEcc::High,
                        _ => QrCodeEcc::Low,
                    });
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
