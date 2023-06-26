use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsQrCode;

use qrcode_generator::QrCodeEcc;

use crate::{widget_entry::{WidgetEntry, WidgetIcon}, textarea_form::TextAreaForm, select_form::SelectForm};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "QR Code Generator",
    short_title: "QR Code",
    description: "Generate QR codes from text",
    path: "/qr-code-generator",
    function: qr_code_generator,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsQrCode> = WidgetIcon {
    icon: BsQrCode,
};

pub fn qr_code_generator(cx: Scope) -> Element {
    let qr_code_value = use_state(cx, || "".to_string());
    let qr_code_error_correction = use_state(cx, || QrCodeEcc::Low);

    let result: String = qrcode_generator::to_svg_to_string(qr_code_value.get(), *qr_code_error_correction.get(), 300, None::<&str>).unwrap();
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
                class: "qr-code",
                dangerous_inner_html: "{result}"
            }
        }
    })
}
