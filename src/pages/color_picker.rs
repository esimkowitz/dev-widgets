#![allow(non_snake_case)]
use dioxus::{html::geometry::{euclid::{Rect, Point2D}, PageSpace}, prelude::*};
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
    use_shared_state_provider(cx, || Color {
        hue: 0f64,
        saturation: 50f64,
        lightness: 50f64,
    });

    cx.render(rsx! {
        div {
            class: "color-picker",
            ColorWheel {}
            ColorView {}
        }
    })
}

fn ColorWheel(cx: Scope) -> Element {
    let color_state = use_shared_state::<Color>(cx).unwrap();
    let dimensions = use_ref(cx, Rect::zero);
    
    cx.render(rsx! {
        div {
            class: "colorwheel-wrapper",
            div {
                class: "colorwheel-gradient",
                onclick: move |event| {
                    let cursor_coordinates = event.data.page_coordinates();
                    let center_coordinates = dimensions.with(|rect| rect.center().cast_unit());
                    color_state.write().hue = cursor_position_to_hue(cursor_coordinates, center_coordinates);
                },
                onmounted: move |cx| {
                    to_owned![dimensions];
                    async move {
                        if let Ok(rect) = cx.get_client_rect().await {
                            dimensions.set(rect);
                        }
                    }
                },
                div {
                    class: "colorwheel-overlay",
                    style: "--colorwheel_cursor_rotation: {hue_to_css_rotation(color_state.read().hue)}deg; --colorwheel_cursor_color: hsl({color_state.read().hue}, 100%, 50%);",
                    div {
                        class: "colorwheel-inner", 
                        onclick: |event| {
                            event.stop_propagation();
                        }
                    },
                    div {
                        class: "colorwheel-cursor"
                    }
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

struct Color {
    hue: f64,
    saturation: f64,
    lightness: f64,
}

fn cursor_position_to_hue(cursor_coordinates: Point2D<f64, PageSpace>, center_coordinates: Point2D<f64, PageSpace>) -> f64 {
    log::info!("cursor_coordinates: {:?}, center_coordinates: {:?}", cursor_coordinates, center_coordinates);
    let vector = cursor_coordinates - center_coordinates;
    let angle = vector.yx().angle_from_x_axis().positive().to_degrees() - 90f64;
    let angle = angle % 360f64;
    log::info!("vector: {:?}, angle: {:?}", vector, angle);
    angle
}

fn hue_to_css_rotation(hue: f64) -> f64 {
    (450f64-hue).abs() % 360f64
}