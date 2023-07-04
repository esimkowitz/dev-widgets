use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsFileEarmarkLock2;

use crate::widget_entry::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "JWT Encoder / Decoder",
    short_title: "JWT",
    description: "Encode and decode JSON Web Tokens",
    path: "/jwt-decoder",
    function: jwt_decoder,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsFileEarmarkLock2> = WidgetIcon {
    icon: BsFileEarmarkLock2,
};

pub fn jwt_decoder(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "jwt-decoder"
        }
    })
}
