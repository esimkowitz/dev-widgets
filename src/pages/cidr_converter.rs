use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, str::FromStr};

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

    let cidr_wildcard = cidr_ref.with(|cidr| {
        match cidr.mask() {
            IpAddr::V4(mask) => {
                let mask = u32::from(mask);
                let wildcard = u32::MAX - mask;
                IpAddr::from(Ipv4Addr::from(wildcard))
            }
            IpAddr::V6(mask) => {
                let mask = u128::from(mask);
                let wildcard = u128::MAX - mask;
                IpAddr::from(Ipv6Addr::from(wildcard))
            }
        }
    });

    const BASE: u128 = 2;
    let addresses_count = cidr_ref.with(|cidr| {
        BASE.pow(match cidr.family() {
            Family::Ipv4 => 32,
            Family::Ipv6 => 128,
        } - u32::from(cidr.network_length())) - 2
    });

    let show_error_state = use_state(cx, || false);
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
                                show_error_state.set(false);
                            },
                            Err(err) => {
                                log::error!("Invalid CIDR: {}, err: {}", cidr, err);
                                show_error_state.set(true);
                            }
                        };
                    });
                }
            }
            TextInput {
                label: "Net Mask",
                value: "{cidr_ref.with(|cidr| cidr.mask().to_string())}",
                readonly: true,
            }
            TextInput {
                label: "Wildcard Bits",
                value: "{cidr_wildcard.to_string()}",
                readonly: true,
            }
            TextInput {
                label: "First IP",
                value: "{cidr_ref.with(|cidr| cidr.first_address())}",
                readonly: true,
            }
            TextInput {
                label: "Last IP",
                value: "{cidr_ref.with(|cidr| cidr.last_address())}",
                readonly: true,
            }
            TextInput {
                label: "Total Addresses",
                value: "{addresses_count}",
                readonly: true,
            }
            div {
                class: "alert alert-warning",
                display: if !show_error_state.get() { "none" } else { "block" },
                "The provided CIDR is invalid."
            }
        }
    })
}
