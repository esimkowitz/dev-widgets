use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconProps, IconShape};

pub fn sidebar_icon<'a, T: IconShape>(cx: Scope) -> Element {
    let icon_cx = Scoped::<'a, IconProps<'a, T>> {
        scope: cx,
        props: &IconProps::<'a, T> {
            width: 20,
            height: 20,
            fill: "currentColor",
            class: "sidebar-icon",
            title: "x",
            icon: todo!(),
        }
    };
    Icon::<T>(&icon_cx)
}
