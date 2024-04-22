use dioxus::prelude::*;
use dioxus_router::prelude::*;
use strum_macros::EnumIter;

pub mod base64_encoder;
pub mod cidr_decoder;

use crate::pages::{Route, WidgetEntry, WidgetRoute};
use base64_encoder::Base64Encoder;
use cidr_decoder::CidrDecoder;

#[derive(Clone, Debug, EnumIter, PartialEq, Routable)]
pub enum EncoderDecoderRoute {
    #[route("/")]
    Index {},
    #[route("/base64")]
    Base64Encoder {},
    #[route("/cidr")]
    CidrDecoder {},
}

fn Index() -> Element {
    rsx! {
        div {
            class: "encoder-decoder"
        }
    }
}

impl WidgetRoute for EncoderDecoderRoute {
    fn get_widget_routes() -> Vec<Route> {
        Self::get_widgets()
            .iter()
            .map(|widget| Route::EncoderDecoder {
                child: widget.clone(),
            })
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
        Self::Index {}
    }
}
