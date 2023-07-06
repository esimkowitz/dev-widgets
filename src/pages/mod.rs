use phf::phf_ordered_map;

use crate::widget_entry::WidgetEntry;

pub mod base64_encoder;
pub mod color_picker;
pub mod date_converter;
pub mod home_page;
pub mod json_yaml_converter;
pub mod number_base_converter;
pub mod qr_code_generator;
pub mod uuid_generator;

pub static WIDGETS: phf::OrderedMap<&str, &'static [WidgetEntry]> = phf_ordered_map! {
    "Encoder" => &[
        base64_encoder::WIDGET_ENTRY,
    ],
    "Converter" => &[
        number_base_converter::WIDGET_ENTRY,
        date_converter::WIDGET_ENTRY,
        json_yaml_converter::WIDGET_ENTRY,
    ],
    "Media" => &[
        color_picker::WIDGET_ENTRY,
    ],
    "Generator" => &[
        qr_code_generator::WIDGET_ENTRY,
        uuid_generator::WIDGET_ENTRY,
    ],
};
