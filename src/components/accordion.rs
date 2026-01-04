#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::pages::Route;

#[component]
pub fn Accordion(
    children: Element,
    title: String,
    icon: Option<Element>,
    is_open: Option<bool>,
    category_route: Option<Route>,
    #[props(default = true)] has_children: bool,
) -> Element {
    let default_open = is_open.unwrap_or(true);
    let mut is_open_signal = use_signal(|| default_open);

    let collapse_class = if *is_open_signal.read() && has_children {
        "collapse bg-base-200 collapse-open"
    } else {
        "collapse bg-base-200"
    };

    rsx! {
        div { class: "accordion-item {collapse_class}",
            div { class: "collapse-title p-2 min-h-0 flex items-center",
                // Category link (icon + title) - no tooltip needed when expanded
                if let Some(route) = category_route.clone() {
                    Link { class: "flex items-center gap-2 grow", to: route,
                        if let Some(icon) = icon {
                            {icon}
                        }
                        {title}
                    }
                } else {
                    div { class: "flex items-center gap-2 grow",
                        if let Some(icon) = icon {
                            {icon}
                        }
                        {title}
                    }
                }
                // Chevron toggle button (only if has children)
                if has_children {
                    button {
                        class: "accordion-toggle-btn",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            is_open_signal.with_mut(|flag| *flag = !*flag);
                        },
                        svg {
                            class: "accordion-chevron",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 20 20",
                            "fill": "currentColor",
                            path {
                                "fill-rule": "evenodd",
                                "d": "M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z",
                                "clip-rule": "evenodd",
                            }
                        }
                    }
                }
            }
            // Collapsible content (only if has children)
            if has_children {
                div { class: "collapse-content p-0",
                    ul { class: "menu w-full", {children} }
                }
            }
        }
    }
}
