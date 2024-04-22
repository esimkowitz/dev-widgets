#![allow(non_snake_case)]
use color_processing::Color;
use dioxus::{
    html::geometry::{
        euclid::{default, Point2D, Rect},
        PageSpace,
    },
    prelude::{SvgAttributes, *},
};
use dioxus_free_icons::icons::bs_icons::BsEyedropper;

use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Color Picker",
    short_title: "Color Picker",
    description: "Pick a color and get its output in different formats",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<BsEyedropper> = WidgetIcon { icon: BsEyedropper };

const SATURATION_BRIGHTNESS_BOX_ID: &str = "saturation-brightness-box";
const COLORWHEEL_ID: &str = "colorwheel";

pub fn ColorPicker() -> Element {
    let mut color_state = use_context_provider(|| {
        Signal::new(ColorPickerState {
            hue: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            alpha: 1.0,
            target: None,
            colorwheel_rect: Rect::zero(),
            saturation_brightness_rect: Rect::zero(),
        })
    });

    let mut process_pointer_event = move |event: Event<PointerData>| {
        color_state.with_mut(|color_state| match color_state.target {
            Some(SATURATION_BRIGHTNESS_BOX_ID) => {
                let page_coordinates = event.data().page_coordinates();
                let cursor_coordinates = Point2D::<f64, PageSpace>::new(
                    page_coordinates.x - color_state.saturation_brightness_rect.min().x,
                    page_coordinates.y - color_state.saturation_brightness_rect.min().y,
                );
                let sv_scale =
                    default::Scale::new(color_state.saturation_brightness_rect.size.width / 100.0);
                let point_sv = cursor_coordinates.cast_unit() / sv_scale;
                color_state.saturation = x_axis_to_saturation(point_sv.x);
                color_state.brightness = y_axis_to_brightness(point_sv.y);
            }
            Some(COLORWHEEL_ID) => {
                let page_coordinates: Point2D<f64, PageSpace> = event.data().page_coordinates();
                let center_coordinates = color_state.colorwheel_rect.center().cast_unit();
                color_state.hue = cursor_position_to_hue(page_coordinates, center_coordinates);
            }
            _ => {}
        })
    };

    let modify_capture_pointer = use_signal(|| {
        move |pointer_id: i32, is_capturing: bool| {
            let eval = eval(match is_capturing {
                true => {
                    r#"
                    let pointer_id = await dioxus.recv();
                    console.log("capturing " + pointer_id);
                    document.getElementById('color-picker').setPointerCapture(pointer_id);
                    "#
                }
                false => {
                    r#"
                    let pointer_id = await dioxus.recv();
                    console.log("releasing " + pointer_id);
                    document.getElementById('color-picker').releasePointerCapture(pointer_id);
                    "#
                }
            });
            eval.send(pointer_id.into()).unwrap();
        }
    });

    rsx! {
        div {
            class: "color-picker",
            id: "color-picker",
            onpointerdown: move |event| {
                let pointerId = event.data().pointer_id();
                event.stop_propagation();
                modify_capture_pointer.with(|modify_capture_pointer| modify_capture_pointer(pointerId, true));
            },
            onpointerup: move |event| {
                let pointerId = event.data().pointer_id();
                modify_capture_pointer.with(|modify_capture_pointer| modify_capture_pointer(pointerId, false));
            },
            ongotpointercapture: move |event| {
                let pointerRect = event.data().page_coordinates();
                if pointerRect.x >= color_state.read().saturation_brightness_rect.min().x
                    && pointerRect.x <= color_state.read().saturation_brightness_rect.max().x
                    && pointerRect.y >= color_state.read().saturation_brightness_rect.min().y
                    && pointerRect.y <= color_state.read().saturation_brightness_rect.max().y
                {
                    color_state.write().target = Some(SATURATION_BRIGHTNESS_BOX_ID);
                } else {
                    color_state.write().target = Some(COLORWHEEL_ID);
                } 
            },
            onlostpointercapture: move |_| {
                color_state.write().target = None;
            },
            onpointermove: move |event| {
                if color_state.read().target.is_some() {
                    process_pointer_event(event);
                }
            },
            div {
                class: "color-picker-inner",
                ColorWheel {}
                SaturationBrightnessBox {}
            }
            ColorView {}
        }
    }
}

fn ColorWheel() -> Element {
    let mut color_state = use_context::<Signal<ColorPickerState>>();

    rsx! {
        div {
            class: "colorwheel-wrapper",
            div {
                class: COLORWHEEL_ID,
                id: COLORWHEEL_ID,
                onmounted: move |event| {
                    async move {
                        if let Ok(rect) = event.get_client_rect().await {
                            color_state.write().colorwheel_rect = rect;
                        }
                    }
                },
                ColorWheelSvg {}
                ColorWheelCursorSvg {
                    hue: color_state.read().hue,
                }
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
        div {
            class: "saturation-brightness-wrapper",
            div {
                class: SATURATION_BRIGHTNESS_BOX_ID,
                id: SATURATION_BRIGHTNESS_BOX_ID,
                onmounted: move |event| {
                    async move {
                        if let Ok(rect) = event.get_client_rect().await {
                            color_state.write().saturation_brightness_rect = rect;
                        }
                    }
                },
                div {
                    class: "saturation-brightness-gradient",
                    style: "background-color: hsl({color_state.read().hue}deg, 100%, 50%);"
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
        svg {
            view_box: "0 0 100 100",
            class: class.unwrap_or("".to_string()),
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
                transform: transform.unwrap_or("".to_string()),
                circle {
                    cx: x.unwrap_or(50f64),
                    cy: y.unwrap_or(3.75 * scale_factor as f64),
                    r: 3.75 * scale_factor as f64,
                    stroke: "url(#cursor-border)",
                    stroke_width: 2 * scale_factor,
                    fill: fill
                }
            }
        }
    }
}

fn ColorView() -> Element {
    let color_state = use_context::<Signal<ColorPickerState>>();
    rsx! {
        div {
            class: "color-view",
            style: "--color-view-background: {color_state.read().get_rgb_string()};"
        }
    }
}

struct ColorPickerState {
    hue: f64,
    saturation: f64,
    brightness: f64,
    alpha: f64,
    target: Option<&'static str>,
    colorwheel_rect: Rect<f64, f64>,
    saturation_brightness_rect: Rect<f64, f64>,
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
