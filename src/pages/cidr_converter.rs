use std::net::{IpAddr, Ipv4Addr};

use cidr::{Family, IpCidr};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsEthernet;
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{
    components::inputs::{SelectForm, SelectFormEnum, TextInput},
    pages::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "CIDR to IP converter",
    short_title: "CIDR",
    description: "Convert a CIDR string to an IP range",
    path: "/cidr-converter",
    function: cidr_converter,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsEthernet> = WidgetIcon { icon: BsEthernet };

pub fn cidr_converter(cx: Scope) -> Element {
    let cidr_ref = use_ref(cx, || {
        IpCidr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0).unwrap()
    });
    cx.render(rsx! {
        div {
            class: "cidr-converter",
            TextInput {
                label: "CIDR",
                value: "{cidr_ref.read().to_string()}",
                onsubmit: |event: Event<FormData>| {
                    let cidr = event.value.clone();
                    log::info!("CIDR: {}", cidr);
                }
            }
        }
    })
}
