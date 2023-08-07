
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod base64_encoder;
pub mod cidr_decoder;

use base64_encoder::Base64Encoder;
use cidr_decoder::CidrDecoder;
use crate::pages::route_trait::WidgetRoute;
use crate::pages::WidgetEntry;

fn EncoderDecoder(cx: Scope) -> Element {
    render! {
        div {
            class: "encoder-decoder"
        }
    }
}

#[derive(Clone, Debug, EnumIter, PartialEq, Routable)]
pub enum EncoderDecoderRoute {
    #[route("/")]
    EncoderDecoder {},
    #[route("/base64")]
    Base64Encoder {},
    #[route("/cidr")]
    CidrDecoder {},
}

impl WidgetRoute for EncoderDecoderRoute {
    fn get_widgets() -> Vec<Self> {
        Self::iter()
            .filter(|route| route.get_widget_entry().is_some())
            .collect()
    }

    fn get_widget_type_string() -> &'static str {
        "Encoder/Decoder"
    }

    fn get_widget_entry(&self) -> Option<&'static WidgetEntry> {
        match self {
            Self::Base64Encoder { .. } => Some(&base64_encoder::WIDGET_ENTRY),
            Self::CidrDecoder { .. } => Some(&cidr_decoder::WIDGET_ENTRY),
            _ => None,
        }
    }
}

impl Default for EncoderDecoderRoute {
    fn default() -> Self {
        Self::EncoderDecoder {}
    }
}
