use std::io::{Error, ErrorKind};

use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsQrCode;
use qrcodegen::{QrCode, QrCodeEcc};

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
    let qrcode = QrCode::encode_text(
        qr_code_value.get().as_str(),
        *qr_code_error_correction.get(),
    )
    .unwrap();
    const QRCODE_SIZE: i64 = 300;
    let path_string = generate_qrcode_path_str(qrcode, QRCODE_SIZE as usize).unwrap();

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
            svg {
                class: "qr-code",
                stroke: "#000",
                height: QRCODE_SIZE,
                width: QRCODE_SIZE,
                view_box: "0 0 {QRCODE_SIZE} {QRCODE_SIZE}",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "#000",
                title {
                    "Generated QR Code"
                }
                path {
                    d: "{path_string}"
                }
            }
        }
    })
}

fn generate_qrcode_path_str(qrcode: QrCode, size: usize) -> Result<String, Error> {
    let mut path_string = String::new();
    let margin_size = 1;

    let s = qrcode.size();

    let data_length = s as usize;

    let data_length_with_margin = data_length + 2 * margin_size;

    let point_size = size / data_length_with_margin;

    if point_size == 0 {
        return Err(Error::new(ErrorKind::Other, "The size is too small"));
    }

    let margin = (size - (point_size * data_length)) / 2;

    for i in 0..s {
        for j in 0..s {
            if qrcode.get_module(j, i) {
                let x = j as usize * point_size + margin;
                let y = i as usize * point_size + margin;

                path_string
                    .push_str(format!("M{x} {y}h{point_size}v{point_size}H{x}V{y}").as_str());
            }
        }
    }
    return Ok(path_string);
}
