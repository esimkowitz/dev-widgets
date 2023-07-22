use cidr::{Family, AnyIpCidr};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsEthernet;
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{pages::{WidgetEntry, WidgetIcon}, components::inputs::{SelectForm, SelectFormEnum}};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "CIDR Encoder/Decoder",
    short_title: "CIDR",
    description: "Encode and decode CIDR addresses to and from IP ranges",
    path: "/cidr-encoder",
    function: cidr_encoder,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsEthernet> = WidgetIcon { icon: BsEthernet };

pub fn cidr_encoder(cx: Scope) -> Element {
    #[allow(clippy::redundant_closure)]
    let family_state = use_state(cx, || IpFamily::default());
    cx.render(rsx! {
        div {
            class: "cidr-encoder",
            SelectForm::<IpFamily> {
                label: "IP Address Family",
                value: *family_state.get(),
                oninput: move |family| {
                    family_state.set(family);
                }
            }
        }
    })
}

#[derive(
    Copy, Clone, Default, Debug, Display, EnumIter, EnumString, Hash, IntoStaticStr, PartialEq,
)]
enum IpFamily {
    #[default]
    IPv4,
    IPv6,
}

impl SelectFormEnum for IpFamily {}