#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Accordion(children: Element, title: String, is_open: Option<bool>) -> Element {
    let default_open = is_open.unwrap_or(false);
    let mut is_open_signal = use_signal(|| default_open);

    let collapse_class = if *is_open_signal.read() {
        "collapse collapse-arrow bg-base-200 collapse-open"
    } else {
        "collapse collapse-arrow bg-base-200"
    };

    rsx! {
        div { class: "accordion-item {collapse_class}",
            div {
                class: "collapse-title p-2 min-h-0 cursor-pointer",
                onclick: move |_| {
                    is_open_signal.with_mut(|flag| *flag = !*flag);
                },
                {title}
            }
            div { class: "collapse-content p-0",
                ul { class: "menu w-full",
                    {children}
                }
            }
        }
    }
}
