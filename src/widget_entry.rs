use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

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
        cx.render(rsx! {
            Icon::<T> {
                class: "icon",
                icon: self.icon,
            }
        })
    }
}