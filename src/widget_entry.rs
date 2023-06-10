use dioxus::prelude::*;

#[derive(PartialEq, Eq)]
pub struct WidgetEntry {
    pub title: &'static str,
    pub description: &'static str,
    pub path: &'static str,
    pub function: fn(cx: Scope) -> Element,
}
