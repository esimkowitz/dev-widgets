use dioxus::prelude::*;

#[derive(PartialEq, Eq, Props, Clone, Copy, Debug)]
pub struct WidgetEntry {
    pub title: &'static str,
    pub description: &'static str,
    pub path: &'static str,
    pub function: for<'a> fn(cx: &'a ScopeState) -> Element<'a>,
}
