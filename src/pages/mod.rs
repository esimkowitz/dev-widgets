use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};
use dioxus_router::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod converter;
pub mod encoder_decoder;
pub mod generator;
pub mod home_page;
pub mod layout;
pub mod media;

use converter::ConverterRoute;
use encoder_decoder::EncoderDecoderRoute;
use generator::GeneratorRoute;
use home_page::HomePage;
use layout::{Container, WidgetView};
use media::MediaRoute;

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

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
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

pub trait WidgetRoute: Routable + IntoEnumIterator + PartialEq + Clone {
    fn get_widget_routes() -> Vec<Route>;

    fn get_widgets() -> Vec<Self> {
        Self::iter()
            .filter(|route| route.get_widget_entry().is_some())
            .collect()
    }

    fn get_widget_title_string(&self) -> Option<&'static str> {
        Some(self.get_widget_entry()?.title)
    }

    fn get_widget_short_title_string(&self) -> Option<&'static str> {
        Some(self.get_widget_entry()?.short_title)
    }

    fn get_widget_description_string(&self) -> Option<&'static str> {
        Some(self.get_widget_entry()?.description)
    }

    fn get_widget_type_string() -> &'static str;

    fn get_widget_entry(&self) -> Option<&'static WidgetEntry>;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct WidgetEntry {
    pub title: &'static str,
    pub short_title: &'static str,
    pub description: &'static str,
    pub icon: fn() -> Element,
}

pub struct WidgetIcon<T: IconShape + Copy> {
    pub(crate) icon: T,
}

impl<T: IconShape + Copy> WidgetIcon<T> {
    pub fn icon(self) -> Element {
        rsx! {
            Icon::<T> {
                class: "icon",
                icon: self.icon,
            }
        }
    }
}
