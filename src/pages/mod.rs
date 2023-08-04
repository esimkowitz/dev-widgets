use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_free_icons::{Icon, IconShape};
use phf::phf_ordered_map;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

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
pub mod layout;

use home_page::HomePage;
use number_base_converter::NumberBaseConverter;
use uuid_generator::UuidGenerator;
use qr_code_generator::QrCodeGenerator;
use hash_generator::HashGenerator;
use date_converter::DateConverter;
use color_picker::ColorPicker;
use cidr_decoder::CidrDecoder;
use base64_encoder::Base64Encoder;
use json_yaml_converter::JsonYamlConverter;
use layout::{Container, WidgetView};

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

#[rustfmt::skip]
#[derive(Clone, Debug, EnumIter, PartialEq, Routable)]
pub enum Route {
    #[layout(Container)]
        #[layout(WidgetView)]
            #[nest("/encoder-decoder")]
                #[route("/")]
                EncoderDecoder {},
                #[route("/base64-encoder")]
                Base64Encoder {},
                #[route("/cidr-decoder")]
                CidrDecoder {},
            #[end_nest]
            #[nest("/converter")]
                #[route("/")]
                Converter {},
                #[route("/number-base-converter")]
                NumberBaseConverter {},
                #[route("/date-converter")]
                DateConverter {},
                #[route("/json-yaml-converter")]
                JsonYamlConverter {},
            #[end_nest]
            #[nest("/media")]
                #[route("/")]
                Media {},
                #[route("/color-picker")]
                ColorPicker {},
            #[end_nest]
            #[nest("/generator")]
                #[route("/")]
                Generator {},
                #[route("/qr-code-generator")]
                QrCodeGenerator {},
                #[route("/uuid-generator")]
                UuidGenerator {},
                #[route("/hash-generator")]
                HashGenerator {},
            #[end_nest]
            #[route("/home")]
            HomePage {},
        #[end_layout]
    #[end_layout]
    #[redirect("/", || Route::HomePage {})]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

fn EncoderDecoder(cx: Scope) -> Element {
    render! {
        div {
            class: "encoder-decoder"
        }
    }
}

fn Converter(cx: Scope) -> Element {
    render! {
        div {
            class: "converter"
        }
    }
}

fn Media(cx: Scope) -> Element {
    render! {
        div {
            class: "media"
        }
    }
}

fn Generator(cx: Scope) -> Element {
    render! {
        div {
            class: "generator"
        }
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}

impl Route {
    pub fn get_widget_entry(&self) -> Option<&WidgetEntry> {
        match self {
            Self::Base64Encoder { .. } => Some(&base64_encoder::WIDGET_ENTRY),
            Self::CidrDecoder { .. } => Some(&cidr_decoder::WIDGET_ENTRY),
            Self::NumberBaseConverter { .. } => Some(&number_base_converter::WIDGET_ENTRY),
            Self::DateConverter { .. } => Some(&date_converter::WIDGET_ENTRY),
            Self::JsonYamlConverter { .. } => Some(&json_yaml_converter::WIDGET_ENTRY),
            Self::ColorPicker { .. } => Some(&color_picker::WIDGET_ENTRY),
            Self::QrCodeGenerator { .. } => Some(&qr_code_generator::WIDGET_ENTRY),
            Self::UuidGenerator { .. } => Some(&uuid_generator::WIDGET_ENTRY),
            Self::HashGenerator { .. } => Some(&hash_generator::WIDGET_ENTRY),
            _ => None,
        }
    }

    pub fn get_widget_type_string(&self) -> Option<&'static str>{
        match self {
            Self::EncoderDecoder { .. } => Some("Encoder/Decoder"),
            Self::Converter { .. } => Some("Converter"),
            Self::Media { .. } => Some("Media"),
            Self::Generator { .. } => Some("Generator"),
            _ => None,
        }
    }

    pub fn get_widget_types() -> Vec<Self> {
        Self::iter().filter(|route| route.get_widget_type_string().is_some()).collect()
    }

    pub fn get_widget_routes_for_type(widget_type: Self) -> Vec<Self> {
        Self::iter().filter(|route| route.is_child_of(&widget_type)).collect()
    }
}

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
