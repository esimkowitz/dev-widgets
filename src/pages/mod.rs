use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};
use dioxus_router::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod route_trait;
pub mod media;
pub mod generator;
pub mod converter;
pub mod home_page;
pub mod layout;
pub mod encoder_decoder;

use home_page::HomePage;
use layout::{Container, WidgetView};
use generator::GeneratorRoute;
use converter::ConverterRoute;
use encoder_decoder::EncoderDecoderRoute;
use media::MediaRoute;

use self::route_trait::WidgetRoute;

#[rustfmt::skip]
#[derive(Clone, Debug, EnumIter, PartialEq, Routable)]
pub enum Route {
    #[layout(Container)]
        #[layout(WidgetView)]
            #[child("/encoder-decoder")]
            EncoderDecoder {
                child: EncoderDecoderRoute,
            },
            #[child("/media")]
            Media {
                child: MediaRoute,
            },
            #[child("/converter")]
            Converter {
                child: ConverterRoute,
            },
            #[child("/generator")]
            Generator {
                child: GeneratorRoute,
            },
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
    pub fn get_widget_entry(&self) -> Option<&'static WidgetEntry> {
        match self {
            Self::EncoderDecoder { child } => child.get_widget_entry(),
            Self::Converter { child } => child.get_widget_entry(),
            Self::Media { child } => child.get_widget_entry(),
            Self::Generator { child } => child.get_widget_entry(),
            _ => None,
        }
    }

    pub fn get_widget_type_string(&self) -> Option<&'static str> {
        match self {
            Self::EncoderDecoder { .. } => Some(EncoderDecoderRoute::get_widget_type_string()),
            Self::Converter { .. } => Some(ConverterRoute::get_widget_type_string()),
            Self::Media { .. } => Some(MediaRoute::get_widget_type_string()),
            Self::Generator { .. } => Some(GeneratorRoute::get_widget_type_string()),
            _ => None,
        }
    }

    pub fn get_widgets(&self) -> Vec<Self> {
        match self {
            Self::EncoderDecoder { .. } => EncoderDecoderRoute::get_widget_routes(),
            Self::Converter { .. } => ConverterRoute::get_widget_routes(),
            Self::Media { .. } => MediaRoute::get_widget_routes(),
            Self::Generator { .. } => GeneratorRoute::get_widget_routes(),
            _ => vec![],
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct WidgetEntry {
    pub title: &'static str,
    pub short_title: &'static str,
    pub description: &'static str,
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
