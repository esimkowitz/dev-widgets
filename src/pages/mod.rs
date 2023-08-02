use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};
use phf::phf_ordered_map;

pub mod base64_encoder;
pub mod cidr_decoder;
pub mod color_picker;
pub mod date_converter;
pub mod hash_generator;
pub mod home_page;
pub mod json_yaml_converter;
pub mod number_base_converter;
pub mod qr_code_generator;
pub mod uuid_generator;

pub static WIDGETS: phf::OrderedMap<&str, &[WidgetEntry]> = phf_ordered_map! {
    "Encoder/Decoder" => &[
        base64_encoder::WIDGET_ENTRY,
        cidr_decoder::WIDGET_ENTRY,
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
        hash_generator::WIDGET_ENTRY,
    ],
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct WidgetEntry {
    pub title: &'static str,
    pub short_title: &'static str,
    pub description: &'static str,
    pub path: &'static str,
    pub function: fn(cx: Scope) -> Element,
    pub icon: fn(cx: Scope) -> Element,
}

pub struct WidgetIcon<T: IconShape + Copy> {
    pub(crate) icon: T,
}

impl<T: IconShape + Copy> WidgetIcon<T> {
    pub fn icon<'a>(&'a self, cx: Scope<'a>) -> Element<'a> {
        render! {
            Icon::<T> {
                class: "icon",
                icon: self.icon,
            }
        }
    }
}
