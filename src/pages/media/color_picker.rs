#![allow(non_snake_case)]
use dioxus::{html::geometry::{euclid::{Rect, Point2D}, PageSpace}, prelude::*};
use dioxus_free_icons::icons::bs_icons::BsEyedropper;

use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Color Picker",
    short_title: "Color Picker",
    description: "Pick a color and get its output in different formats",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<BsEyedropper> = WidgetIcon { icon: BsEyedropper };

pub fn ColorPicker() -> Element {
    use_context_provider(|| Color {
        hue: 0f64,
        saturation: 50f64,
        lightness: 50f64,
    });

    rsx! {
        div {
            class: "color-picker",
            ColorWheel {}
            ColorView {}
        }
    }
}

fn ColorWheel() -> Element {
    let mut color_state = use_context::<Color>();
    let dimensions = use_signal(Rect::<f64, PageSpace>::zero);
    let mut tracking_state = use_signal(|| false);

    let mut process_mouse_event = move |event: Event<MouseData>| {
        let cursor_coordinates = event.data.page_coordinates();
        let center_coordinates = dimensions.with(|rect| rect.center().cast_unit());
        color_state.hue = cursor_position_to_hue(cursor_coordinates, center_coordinates);
    };
    
    rsx! {
        div {
            class: "colorwheel-wrapper",
            onmounted: move |_| {
                to_owned![dimensions];
                async move {
                    // if let Ok(rect) = get_client_rect().await {
                    //     dimensions.set(rect);
                    // }
                }
            },
            onmousedown: move |_| {
                tracking_state.set(true);
            },
            onmouseup: move |_| {
                tracking_state.set(false);
            },
            onmouseleave: move |_| {
                tracking_state.set(false);
            },
            onmousemove: move |event| {
                if *tracking_state.read() {
                    process_mouse_event(event);
                }
            },
            onclick: move |event| {
                process_mouse_event(event);
            },
            ColorWheelSvg {}
            ColorWheelCursorSvg {
                hue: color_state.hue,
            }
        }
    }
}

fn ColorWheelSvg() -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 100",
            class: "colorwheel-svg",
            mask {
                id: "colorwheel-mask",
                circle {
                    cx: 50,
                    cy: 50,
                    r: 50,
                    fill: "white",
                }
                circle {
                    cx: 50,
                    cy: 50,
                    r: 42.5,
                    fill: "black",
                }
            },
            foreignObject {
                x: 0,
                y: 0,
                width: 100,
                height: 100,
                mask: "url(#colorwheel-mask)",
                div {
                    class: "colorwheel-gradient",
                }
            },
        }
    }
}

#[component]
fn ColorWheelCursorSvg(hue: f64) -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 100",
            class: "colorwheel-cursor",
            defs {
                radialGradient {
                    id: "colorwheel-cursor-border",
                    stop {
                        offset: "0%",
                        stop_color: "white",
                        stop_opacity: 0,
                    }
                    stop {
                        offset: "50%",
                        stop_color: "white",
                        stop_opacity: 1,
                    }
                    stop {
                        offset: "90%",
                        stop_color: "white",
                        stop_opacity: 1,
                    }
                    stop {
                        offset: "90%",
                        stop_color: "lightgray",
                        stop_opacity: 1,
                    }
                    stop {
                        offset: "100%",
                        stop_color: "lightgray",
                        stop_opacity: 0,
                    }
                }
            }
            g {
                transform: "rotate({hue_to_css_rotation(hue)} 50 50)",
                circle {
                    cx: 50,
                    cy: 3.75,
                    r: 3.75,
                    stroke: "url(#colorwheel-cursor-border)",
                    stroke_width: 2,
                    fill: "hsl({hue}, 100%, 50%)"
                }
            }
        }
    }
}

fn ColorView() -> Element {
    let color_state = use_context::<Color>();

    let color = color_state;
    rsx! {
        div {
            class: "color-view",
            style: "--color-view-background: hsl({color.hue}deg, {color.saturation}%, {color.lightness}%);"
        }
    }
}

#[derive(Clone)]
struct Color {
    hue: f64,
    saturation: f64,
    lightness: f64,
}

fn cursor_position_to_hue(cursor_coordinates: Point2D<f64, PageSpace>, center_coordinates: Point2D<f64, PageSpace>) -> f64 {
    log::trace!("cursor_coordinates: {:?}, center_coordinates: {:?}", cursor_coordinates, center_coordinates);
    let vector = cursor_coordinates - center_coordinates;
    let angle = vector.yx().angle_from_x_axis().positive().to_degrees() - 90f64;
    let angle = angle % 360f64;
    log::trace!("vector: {:?}, angle: {:?}", vector, angle);
    angle
}

fn hue_to_css_rotation(hue: f64) -> f64 {
    (450f64-hue).abs() % 360f64
}