use std::{net::{IpAddr, Ipv4Addr}, str::FromStr};

use cidr::{Family, IpCidr};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsEthernet;

use crate::{
    components::inputs::TextInput,
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
                onsubmit: |event: String| {
                    let cidr = event;
                    log::info!("CIDR: {}", cidr);
                    let cidr = cidr.trim();
                    cidr_ref.with_mut(|cidr_obj| {
                        match IpCidr::from_str(cidr) {
                            Ok(cidr) => {
                                *cidr_obj = cidr;
                            },
                            Err(_) => {
                                log::error!("Invalid CIDR: {}", cidr);
                            }
                        };
                    });
                }
            }
        }
    })
}
