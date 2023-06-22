use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

pub struct SidebarIcon<T: IconShape + Copy> {
    pub(crate) icon: T,
}

impl<T: IconShape + Copy> SidebarIcon<T> {
    pub fn sidebar_icon<'a>(&'a self, cx: Scope<'a>) -> Element<'a> {
        cx.render(rsx! {
            Icon::<T> {
                icon: self.icon,
            }
        })
    }
}
