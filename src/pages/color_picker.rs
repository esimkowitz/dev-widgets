#![allow(non_snake_case)]
use color_processing::Color;
use dioxus::{
    html::geometry::{
        euclid::{Point2D, Rect},
        PageSpace,
    },
    prelude::*,
};
use dioxus_free_icons::icons::bs_icons::BsEyedropper;

use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Color Picker",
    short_title: "Color Picker",
    description: "Pick a color and get its output in different formats",
    path: "/color-picker",
    function: ColorPicker,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsEyedropper> = WidgetIcon { icon: BsEyedropper };

pub fn ColorPicker(cx: Scope) -> Element {
    use_shared_state_provider(cx, || ColorPickerState {
        color: Box::new(Color::new_rgb(255, 0, 0)),
    });

    cx.render(rsx! {
        div {
            class: "color-picker",
            ColorWheel {}
            ColorView {}
            SaturationBrightnessBox {}
        }
    })
}

fn ColorWheel(cx: Scope) -> Element {
    let color_state = use_shared_state::<ColorPickerState>(cx).unwrap();
    let dimensions = use_ref(cx, Rect::zero);
    let tracking_state = use_state(cx, || false);

    let process_mouse_event = move |event: Event<MouseData>| {
        let cursor_coordinates = event.data.page_coordinates();
        let center_coordinates = dimensions.with(|rect| rect.center().cast_unit());
        color_state.write().update_hue(cursor_position_to_hue(cursor_coordinates, center_coordinates));
    };

    cx.render(rsx! {
        div {
            class: "colorwheel-wrapper",
            onmounted: move |cx| {
                to_owned![dimensions];
                async move {
                    if let Ok(rect) = cx.get_client_rect().await {
                        dimensions.set(rect);
                    }
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
                if *tracking_state.get() {
                    process_mouse_event(event);
                }
            },
            onclick: move |event| {
                process_mouse_event(event);
            },
            ColorWheelSvg {}
            ColorWheelCursorSvg {
                hue: color_state.read().hue(),
            }
        }
    })
}

fn ColorWheelSvg(cx: Scope) -> Element {
    cx.render(rsx! {
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
    })
}

#[inline_props]
fn ColorWheelCursorSvg(cx: Scope, hue: f64) -> Element {
    cx.render(rsx! {
        CursorPrimitiveSvg {
            class: "colorwheel-cursor",
            fill: "hsl({hue}deg, 100%, 50%)",
            transform: "rotate({hue_to_css_rotation(*hue)} 50 50)",
        }
    })
}

fn SaturationBrightnessBox(cx: Scope) -> Element {
    let color_state = use_shared_state::<Color>(cx).unwrap();
    let dimensions = use_ref(cx, Rect::zero);
    cx.render(rsx! {
        div {
            class: "saturation-brightness-box",
            onmounted: move |cx| {
                to_owned![dimensions];
                async move {
                    if let Ok(rect) = cx.get_client_rect().await {
                        dimensions.set(rect);
                    }
                }
            },
            div {
                class: "saturation-brightness-gradient",
                style: "background-color: hsl({color_state.read().hue()}deg, 100%, 50%);"
            }
            CursorPrimitiveSvg {
                class: "saturation-brightness-cursor",
                fill: "{color_state.read().color.to_css_string()}",
                x: color_state.read().color.,
                y: color_state.read().lightness,
            }
        }
    })
}

#[inline_props]
fn CursorPrimitiveSvg<'a>(
    cx: Scope<'a>,
    x: Option<f64>,
    y: Option<f64>,
    class: Option<&'a str>,
    fill: &'a str,
    transform: Option<&'a str>,
) -> Element<'a> {
    cx.render(rsx! {
        svg {
            view_box: "0 0 100 100",
            class: "{class.unwrap_or(\"\")}",
            defs {
                radialGradient {
                    id: "cursor-border",
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
                transform: "{transform.unwrap_or(\"\")}",
                circle {
                    cx: x.unwrap_or(50f64),
                    cy: y.unwrap_or(3.75),
                    r: 3.75,
                    stroke: "url(#cursor-border)",
                    stroke_width: 2,
                    fill: "{fill}"
                }
            }
        }
    })
}

fn ColorView(cx: Scope) -> Element {
    let color_state = use_shared_state::<Color>(cx).unwrap();

    let color = color_state.read();
    cx.render(rsx! {
        div {
            class: "color-view",
            style: "--color-view-background: hsl({color.hue}deg, {color.saturation}%, {color.lightness}%);"
        }
    })
}

struct ColorPickerState {
    color: Box<Color>,
}

impl ColorPickerState {
    fn update_hue(&mut self, hue: f64) {
        let (_, saturation, luminance, alpha) = self.color.get_hsla();
        *self.color = Color::new_hsla(hue, saturation, luminance, alpha);
    }

    fn hue(&self) -> f64 {
        self.color.get_hsla().0
    }
}

fn cursor_position_to_hue(
    cursor_coordinates: Point2D<f64, PageSpace>,
    center_coordinates: Point2D<f64, PageSpace>,
) -> f64 {
    log::trace!(
        "cursor_coordinates: {:?}, center_coordinates: {:?}",
        cursor_coordinates,
        center_coordinates
    );
    let vector = cursor_coordinates - center_coordinates;
    let angle = vector.yx().angle_from_x_axis().positive().to_degrees() - 90f64;
    let angle = angle % 360f64;
    log::trace!("vector: {:?}, angle: {:?}", vector, angle);
    angle
}

fn hue_to_css_rotation(hue: f64) -> f64 {
    (450f64 - hue).abs() % 360f64
}
