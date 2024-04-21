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

pub fn ColorPicker() -> Element {
    use_context_provider(|| {
        Signal::new(ColorPickerState {
            hue: 0.0,
            saturation: 1.0,
            brightness: 1.0,
            alpha: 1.0,
        })
    });

    rsx! {
        div {
            class: "color-picker",
            ColorWheel {}
            SaturationBrightnessBox {}
            ColorView {}
        }
    }
}

fn ColorWheel() -> Element {
    let mut color_state = use_context::<Signal<ColorPickerState>>();
    let mut tracking_state = use_signal(|| false);
    let dimensions = use_signal(Rect::<f64, f64>::zero);

    let mut process_pointer_event = move |event: Event<PointerData>| {
        let page_coords = event.data().page_coordinates();
        let center_coordinates = dimensions.with(|rect| rect.center().cast_unit());
        color_state.write().hue = cursor_position_to_hue(page_coords, center_coordinates);
    };

    let modify_capture_pointer = use_signal(|| {
        move |pointer_id: i32, is_capturing: bool| {
            log::info!("modifying capture pointer Colorwheel");
            let eval = eval(match is_capturing {
                true => {
                    r#"
                    let pointer_id = await dioxus.recv();
                    console.log("capturing " + pointer_id);
                    document.getElementById('colorwheel').setPointerCapture(pointer_id);
                    "#
                },false => {
                    r#"
                    let pointer_id = await dioxus.recv();
                    console.log("releasing " + pointer_id);
                    document.getElementById('colorwheel').releasePointerCapture(pointer_id);
                    "#
                },
            });
            eval.send(pointer_id.into()).unwrap();
        }
    });

    rsx! {
        div {
            class: "colorwheel-wrapper",
            div {
                class: "colorwheel",
                id: "colorwheel",
                onmounted: move |cx| {
                    to_owned![dimensions];
                    async move {
                        if let Ok(rect) = cx.get_client_rect().await {
                            dimensions.set(rect);
                        }
                    }
                },
                onpointerdown: move |event| {
                    event.stop_propagation();
                    let pointerId = event.data().pointer_id();
                    log::info!("pointerdown, {}", pointerId);
                    modify_capture_pointer.with(|modify_capture_pointer| modify_capture_pointer(pointerId, true));
                    process_pointer_event(event);
                },
                onpointerup: move |event| {
                    event.stop_propagation();
                    let pointerId = event.data().pointer_id();
                    log::info!("pointerup, {}", pointerId);
                    modify_capture_pointer.with(|modify_capture_pointer| modify_capture_pointer(pointerId, false));
                },
                ongotpointercapture: move |_| {
                    log::info!("gotpointercapture");
                    tracking_state.set(true);
                },
                onlostpointercapture: move |_| {
                    log::info!("lostpointercapture");
                    tracking_state.set(false);
                },
                onpointermove: move |event| {
                    event.stop_propagation();
                    if *tracking_state.read() {
                        process_pointer_event(event);
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
    let mut tracking_state = use_signal(|| false);
    let dimensions = use_signal(Rect::<f64, f64>::zero);

    let mut process_pointer_event = move |event: Event<PointerData>| {
        let cursor_coordinates = event.data().element_coordinates();
        log::trace!("cursor_coordinates: {:?}", cursor_coordinates);
        dimensions.with(|dimensions| {
            log::trace!(
                "dimensions_min: {:?}, dimensions_max: {:?}, dimensions_size: {:?}",
                dimensions.min(),
                dimensions.max(),
                dimensions.size
            );
        });
        let sv_scale = default::Scale::new(dimensions.read().size.width / 100.0);
        let point_sv = cursor_coordinates.cast_unit() / sv_scale;
        log::trace!("point_sv: {:?}", point_sv);
        color_state.with_mut(|color_state| {
            color_state.saturation = x_axis_to_saturation(point_sv.x);
            color_state.brightness = y_axis_to_brightness(point_sv.y);
        });
    };

    let modify_capture_pointer = use_signal(|| {
        move |pointer_id: i32, is_capturing: bool| {
            log::info!("modifying capture pointer S/B");
            let eval = eval(match is_capturing {
                true => {
                    r#"
                    let pointer_id = await dioxus.recv();
                    console.log("capturing " + pointer_id);
                    document.getElementById('saturation-brightness-box').setPointerCapture(pointer_id);
                    "#
                },false => {
                    r#"
                    let pointer_id = await dioxus.recv();
                    console.log("releasing " + pointer_id);
                    document.getElementById('saturation-brightness-box').releasePointerCapture(pointer_id);
                    "#
                },
            });
            eval.send(pointer_id.into()).unwrap();
        }
    });

    rsx! {
        div {
            class: "saturation-brightness-wrapper",
            div {
                class: "saturation-brightness-box",
                id: "saturation-brightness-box",
                onmounted: move |cx| {
                    to_owned![dimensions];
                    async move {
                        if let Ok(rect) = cx.get_client_rect().await {
                            dimensions.set(rect);
                        }
                    }
                },
                onpointerdown: move |event| {
                    event.stop_propagation();
                    let pointerId = event.data().pointer_id();
                    log::info!("pointerdown, {}", pointerId);
                    modify_capture_pointer.with(|modify_capture_pointer| modify_capture_pointer(pointerId, true));
                    process_pointer_event(event);
                },
                onpointerup: move |event| {
                    event.stop_propagation();
                    let pointerId = event.data().pointer_id();
                    log::info!("pointerup, {}", pointerId);
                    modify_capture_pointer.with(|modify_capture_pointer| modify_capture_pointer(pointerId, false));
                },
                ongotpointercapture: move |_| {
                    log::info!("gotpointercapture");
                    tracking_state.set(true);
                },
                onlostpointercapture: move |_| {
                    log::info!("lostpointercapture");
                    tracking_state.set(false);
                },
                onpointermove: move |event| {
                    event.stop_propagation();
                    if *tracking_state.read() {
                        process_pointer_event(event);
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
) -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 100",
            class: class.unwrap_or("\\".to_string()),
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
                transform: transform.unwrap_or("\\".to_string()),
                circle {
                    cx: x.unwrap_or(50f64),
                    cy: y.unwrap_or(3.75),
                    r: 3.75,
                    stroke: "url(#cursor-border)",
                    stroke_width: 2,
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
