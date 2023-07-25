use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use cidr::{Family, IpCidr};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsEthernet;

use crate::{
    components::inputs::{TextAreaForm, TextInput},
    pages::{WidgetEntry, WidgetIcon},
    utils::add_number_delimiters,
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "CIDR Decoder",
    short_title: "CIDR",
    description: "Decode Classless Inter-Domain Routing (CIDR) notation to IP address range",
    path: "/cidr-decoder",
    function: cidr_decoder,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsEthernet> = WidgetIcon { icon: BsEthernet };

pub fn cidr_decoder(cx: Scope) -> Element {
    let cidr_ref = use_ref(cx, || {
        IpCidr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0).unwrap()
    });

    let cidr_input_ref = use_ref(cx, || cidr_ref.with(|cidr| cidr.to_string()));

    let cidr_description = cidr_ref.with(|cidr| {
        let mut description = String::new();
        description.push_str(&format!("Netmask: {}\n", cidr.mask()));
        description.push_str(&format!(
            "Wildcard: {}\n",
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
        ));
        description.push_str(&format!("First IP: {}\n", cidr.first_address()));
        description.push_str(&format!("Last IP: {}\n", cidr.last_address()));
        description.push_str(&format!("Total Addresses: {}\n", {
            const BASE: u128 = 2;
            let power = match cidr.family() {
                Family::Ipv4 => 32,
                Family::Ipv6 => 128,
            } - u32::from(cidr.network_length());

            if power == 128 {
                // This is too big to fit in a u128, so we have to hardcode it or use a non-std u256 crate.
                "340,282,366,920,938,463,463,374,607,431,768,211,456".to_string()
            } else {
                add_number_delimiters(BASE.pow(power).to_string(), ',', 3)
            }
        }));
        description
    });

    let show_error_state = use_state(cx, || false);
    cx.render(rsx! {
        div {
            class: "cidr-decoder",
            TextInput {
                label: "CIDR",
                value: "{cidr_input_ref.with(|cidr_str| cidr_str.to_string())}",
                oninput: |event: Event<FormData>| {
                    let cidr = event.value.clone();
                    log::info!("CIDR: {}", cidr);
                    cidr_input_ref.with_mut(|cidr_input| {
                        *cidr_input = cidr.clone();
                    });
                    let cidr_trim = cidr.trim();
                    if let Ok(cidr_valid) = IpCidr::from_str(cidr_trim) {
                        cidr_ref.with_mut(|cidr_obj| {
                            *cidr_obj = cidr_valid;
                            show_error_state.set(false);
                        });
                    } else {
                        show_error_state.set(true);
                    }
                }
            }
            div {
                class: "alert alert-warning",
                hidden: !show_error_state.get(),
                "The provided CIDR is invalid."
            }
            TextAreaForm {
                label: "Description",
                readonly: true,
                value: "{cidr_description}",
            }
        }
    })
}
