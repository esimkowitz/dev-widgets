#![allow(non_snake_case)]
use std::rc::Rc;

use color_processing::Color;
use dioxus::{
    html::geometry::{
        euclid::{default, Rect},
        PixelsRect,
    },
    prelude::*,
};
use dioxus_free_icons::icons::fa_solid_icons::FaPalette;
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{
    components::inputs::{SelectForm, SelectFormEnum, TextInput},
    pages::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Color Picker",
    short_title: "Color Picker",
    description: "Pick a color and get its output in different formats",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<FaPalette> = WidgetIcon { icon: FaPalette };

const SATURATION_BRIGHTNESS_BOX_ID: &str = "saturation-brightness-box";
const COLORWHEEL_ID: &str = "colorwheel";

pub fn ColorPicker() -> Element {
    // Which element the active drag started on; `None` when not dragging
    let mut target = use_signal(|| None::<&'static str>);
    let mut color_state = use_context_provider(|| {
        Signal::new(ColorPickerState {
            hue: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            alpha: 1.0,
            colorwheel: None,
            saturation_brightness_box: None,
            colorwheel_rect: Rect::zero(),
            saturation_brightness_rect: Rect::zero(),
        })
    });

    let mut process_pointer_event = move |point: default::Point2D<f64>| {
        color_state.with_mut(|color_state| match *target.read() {
            Some(SATURATION_BRIGHTNESS_BOX_ID) => {
                let rect = color_state.saturation_brightness_rect.to_untyped();
                let x = ((point.x - rect.min_x()) / rect.width() * 100.0).clamp(0.0, 100.0);
                let y = ((point.y - rect.min_y()) / rect.height() * 100.0).clamp(0.0, 100.0);
                color_state.saturation = x_axis_to_saturation(x);
                color_state.brightness = y_axis_to_brightness(y);
            }
            Some(COLORWHEEL_ID) => {
                let center = color_state.colorwheel_rect.to_untyped().center();
                color_state.hue = cursor_position_to_hue(point, center);
            }
            _ => {}
        })
    };

    let onpointerdown = move |event: PointerEvent| async move {
        event.stop_propagation();
        event.prevent_default();
        let point = event.client_coordinates().to_untyped();

        // Re-measure on every press: the content area scrolls and the sidebar
        // resizes, so rects captured at mount time go stale.
        let (colorwheel, saturation_brightness_box) = {
            let state = color_state.read();
            (
                state.colorwheel.clone(),
                state.saturation_brightness_box.clone(),
            )
        };
        if let Some(el) = colorwheel {
            if let Ok(rect) = el.get_client_rect().await {
                color_state.write().colorwheel_rect = rect;
            }
        }
        if let Some(el) = saturation_brightness_box {
            if let Ok(rect) = el.get_client_rect().await {
                color_state.write().saturation_brightness_rect = rect;
            }
        }

        let hit = hit_test(point, &color_state.read());
        target.set(hit);
        process_pointer_event(point);
    };

    let onpointermove = move |event: PointerEvent| {
        if target.read().is_some() {
            process_pointer_event(event.client_coordinates().to_untyped());
        }
    };
    let end_drag = move |_: PointerEvent| target.set(None);

    rsx! {
        div { class: "color-picker",
            // Touch pointers are implicitly captured by the element they started on,
            // so touch moves always bubble up to here.
            div {
                class: "color-picker-inner",
                onpointerdown,
                onpointermove,
                onpointerup: end_drag,
                onpointercancel: end_drag,
                ColorWheel {}
                SaturationBrightnessBox {}
            }
            // Mouse pointers aren't captured, so while dragging, cover the viewport
            // to keep receiving events after the cursor leaves the wheel.
            if target.read().is_some() {
                div {
                    class: "fixed inset-0 z-50 cursor-grabbing",
                    onpointermove,
                    onpointerup: end_drag,
                    onpointercancel: end_drag,
                }
            }
            ColorView {}
        }
    }
}

/// Decide which control a press at `point` (client coordinates) should drag
fn hit_test(point: default::Point2D<f64>, state: &ColorPickerState) -> Option<&'static str> {
    if state
        .saturation_brightness_rect
        .to_untyped()
        .contains(point)
    {
        return Some(SATURATION_BRIGHTNESS_BOX_ID);
    }
    let wheel = state.colorwheel_rect.to_untyped();
    if (point - wheel.center()).length() <= wheel.width() / 2.0 {
        return Some(COLORWHEEL_ID);
    }
    None
}

fn ColorWheel() -> Element {
    let mut color_state = use_context::<Signal<ColorPickerState>>();

    rsx! {
        div { class: "colorwheel-wrapper",
            div {
                class: COLORWHEEL_ID,
                id: COLORWHEEL_ID,
                onmounted: move |event| {
                    async move {
                        color_state.write().colorwheel = Some(event.data());
                        if let Ok(rect) = event.get_client_rect().await {
                            color_state.write().colorwheel_rect = rect;
                        }
                    }
                },
                ColorWheelSvg {}
                ColorWheelCursorSvg { hue: color_state.read().hue }
            }
        }
    }
}

fn ColorWheelSvg() -> Element {
    rsx! {
        svg { view_box: "0 0 100 100", class: "colorwheel-svg",
            mask { id: "colorwheel-mask",
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
            }
            foreignObject {
                x: 0,
                y: 0,
                width: 100,
                height: 100,
                mask: "url(#colorwheel-mask)",
                div { class: "colorwheel-gradient" }
            }
        }
    }
}

#[component]
fn ColorWheelCursorSvg(hue: f64) -> Element {
    rsx! {
        CursorPrimitiveSvg {
            class: "colorwheel-cursor",
            fill: "hsl({hue}deg, 100%, 50%)",
            transform: "rotate({hue_to_css_rotation(hue)} 50 50)",
        }
    }
}

fn SaturationBrightnessBox() -> Element {
    let mut color_state = use_context::<Signal<ColorPickerState>>();

    rsx! {
        div { class: "saturation-brightness-wrapper",
            div {
                class: SATURATION_BRIGHTNESS_BOX_ID,
                id: SATURATION_BRIGHTNESS_BOX_ID,
                onmounted: move |event| {
                    async move {
                        color_state.write().saturation_brightness_box = Some(event.data());
                        if let Ok(rect) = event.get_client_rect().await {
                            color_state.write().saturation_brightness_rect = rect;
                        }
                    }
                },
                div {
                    class: "saturation-brightness-gradient",
                    style: "background-color: hsl({color_state.read().hue}deg, 100%, 50%);",
                }
                CursorPrimitiveSvg {
                    class: "saturation-brightness-cursor",
                    fill: "{color_state.read().get_rgb_string()}",
                    x: saturation_to_x_axis(color_state.read().saturation),
                    y: brightness_to_y_axis(color_state.read().brightness),
                    scale_factor: 2,
                }
            }
        }
    }
}

#[component]
fn CursorPrimitiveSvg(
    x: Option<f64>,
    y: Option<f64>,
    class: Option<String>,
    fill: String,
    transform: Option<String>,
    scale_factor: Option<i64>,
) -> Element {
    let scale_factor = scale_factor.unwrap_or(1);
    rsx! {
        svg { view_box: "0 0 100 100", class: class.unwrap_or("".to_string()),
            defs {
                radialGradient { id: "cursor-border",
                    stop { offset: "0%", stop_color: "white", stop_opacity: 0 }
                    stop { offset: "50%", stop_color: "white", stop_opacity: 1 }
                    stop { offset: "90%", stop_color: "white", stop_opacity: 1 }
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
            g { transform: transform.unwrap_or("".to_string()),
                circle {
                    cx: x.unwrap_or(50f64),
                    cy: y.unwrap_or(3.75 * scale_factor as f64),
                    r: 3.75 * scale_factor as f64,
                    stroke: "url(#cursor-border)",
                    stroke_width: 2 * scale_factor,
                    fill,
                }
            }
        }
    }
}

fn ColorView() -> Element {
    let mut color_format = use_signal(ColorFormat::default);
    let color_state = use_context::<Signal<ColorPickerState>>();
    let color = color_state.read().get_color();
    let rgb_string = color.to_rgb_string();
    let color_text = match *color_format.read() {
        ColorFormat::RGB => rgb_string.clone(),
        ColorFormat::HSL => color.to_hsl_string(),
        ColorFormat::HSV => color.to_hsv_string(),
        ColorFormat::HEX => color.to_hex_string(),
        ColorFormat::HWB => color.to_hwb_string(),
        ColorFormat::CMYK => color.to_cmyk_string(),
    };
    rsx! {
        div { class: "color-view",
            div {
                class: "color-view-display",
                style: "--color-view-background: {rgb_string};",
            }
            TextInput { label: "Color", value: color_text, readonly: true }
            SelectForm::<ColorFormat> {
                label: "Color Format",
                oninput: move |new_format: ColorFormat| { color_format.set(new_format) },
                value: *color_format.read(),
            }
        }
    }
}

struct ColorPickerState {
    hue: f64,
    saturation: f64,
    brightness: f64,
    alpha: f64,
    colorwheel: Option<Rc<MountedData>>,
    saturation_brightness_box: Option<Rc<MountedData>>,
    colorwheel_rect: PixelsRect,
    saturation_brightness_rect: PixelsRect,
}

#[derive(
    Copy, Clone, Default, Debug, Display, EnumIter, EnumString, Hash, IntoStaticStr, PartialEq,
)]
#[allow(clippy::upper_case_acronyms)]
enum ColorFormat {
    #[default]
    RGB,
    HSL,
    HSV,
    HEX,
    HWB,
    CMYK,
}

impl SelectFormEnum for ColorFormat {}

impl From<ColorFormat> for String {
    fn from(color_format: ColorFormat) -> Self {
        color_format.to_string()
    }
}

impl ColorPickerState {
    fn get_color(&self) -> Color {
        Color::new_hsva(self.hue, self.saturation, self.brightness, self.alpha)
    }

    fn get_rgb_string(&self) -> String {
        self.get_color().to_rgb_string()
    }
}

fn cursor_position_to_hue(
    cursor_coordinates: default::Point2D<f64>,
    center_coordinates: default::Point2D<f64>,
) -> f64 {
    tracing::trace!(
        "cursor_coordinates: {:?}, center_coordinates: {:?}",
        cursor_coordinates,
        center_coordinates
    );
    let vector = cursor_coordinates - center_coordinates;
    let angle = vector.yx().angle_from_x_axis().positive().to_degrees() - 90f64;
    let angle = angle % 360f64;
    tracing::trace!("vector: {:?}, angle: {:?}", vector, angle);
    angle
}

fn hue_to_css_rotation(hue: f64) -> f64 {
    (450f64 - hue).abs() % 360f64
}

fn saturation_to_x_axis(saturation: f64) -> f64 {
    saturation * 100f64
}

fn brightness_to_y_axis(brightness: f64) -> f64 {
    100f64 - (brightness * 100f64)
}

fn x_axis_to_saturation(x: f64) -> f64 {
    x / 100f64
}

fn y_axis_to_brightness(y: f64) -> f64 {
    (100f64 - y) / 100f64
}
