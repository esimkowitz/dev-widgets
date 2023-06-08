use dioxus::prelude::*;

#[derive(PartialEq, Eq)]
pub struct WidgetEntry {
    pub title: &'static str,
    pub description: &'static str,
    pub widget: Widget,
    pub function: fn(cx: Scope) -> Element,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Widget {
    Base64Encoder,
    DateConverter,
    NumberBaseConverter,
    JsonYamlConverter,
    ColorPicker,
    Home,
}