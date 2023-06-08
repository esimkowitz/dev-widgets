use dioxus::prelude::*;

#[derive(PartialEq, Eq)]
pub struct WidgetEntry {
    pub title: &'static str,
    pub description: &'static str,
    pub widget_type: WidgetType,
    pub widget: Widget,
    pub function: fn(cx: Scope) -> Element,
}

#[derive(PartialEq, Eq, Hash)]
pub enum WidgetType {
    Converter,
    Encoder,
    Media,
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