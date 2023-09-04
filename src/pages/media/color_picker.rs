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
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsEyedropper> = WidgetIcon { icon: BsEyedropper };

pub fn ColorPicker(cx: Scope) -> Element {
    use_shared_state_provider(cx, || ColorPickerState {
        hue: 0.0,
        saturation: 1.0,
        brightness: 1.0,
        alpha: 1.0,
    });

    render! {
        div {
            class: "color-picker",
            ColorWheel {}
            SaturationBrightnessBox {}
            ColorView {}
        }
    }
}

fn ColorWheel(cx: Scope) -> Element {
    let color_state = use_shared_state::<ColorPickerState>(cx).unwrap();
    let dimensions = use_ref(cx, Rect::zero);
    let tracking_state = use_state(cx, || false);

    let process_pointer_event = move |event: Event<PointerData>| {
        let center_coordinates = dimensions.with(|rect| rect.center().cast_unit());
        let cursor_coordinates = Point2D::<f64, dioxus_html::geometry::PageSpace>::new(event.data.page_x.into(), event.data.page_y.into());
        color_state.write().hue = cursor_position_to_hue(cursor_coordinates, center_coordinates);
    };

    let create_eval = use_eval(cx);

    let modify_capture_pointer_ref = use_ref(cx, || {
        to_owned![create_eval];
        move |pointer_id: i32, is_capturing: bool| {
            let eval = create_eval(match is_capturing {
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
            }).unwrap();
            eval.send(pointer_id.into()).unwrap();
        }
    });

    render! {
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
                    let pointerId = event.data.pointer_id;
                    log::info!("pointerdown, {}", pointerId);
                    modify_capture_pointer_ref.with(|modify_capture_pointer| modify_capture_pointer(pointerId, true));
                    process_pointer_event(event);
                },
                onpointerup: move |event| {
                    event.stop_propagation();
                    let pointerId = event.data.pointer_id;
                    log::info!("pointerup, {}", pointerId);
                    modify_capture_pointer_ref.with(|modify_capture_pointer| modify_capture_pointer(pointerId, false));
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
                    log::info!("pointermove {}", event.data.client_x);
                    if *tracking_state.get() {
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

fn ColorWheelSvg(cx: Scope) -> Element {
    render! {
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

#[inline_props]
fn ColorWheelCursorSvg(cx: Scope, hue: f64) -> Element {
    render! {
        CursorPrimitiveSvg {
            class: "colorwheel-cursor",
            fill: "hsl({hue}deg, 100%, 50%)",
            transform: "rotate({hue_to_css_rotation(*hue)} 50 50)",
        }
    }
}

fn SaturationBrightnessBox(cx: Scope) -> Element {
    let color_state = use_shared_state::<ColorPickerState>(cx).unwrap();
    let tracking_state = use_state(cx, || false);
    let dimensions = use_ref(cx, Rect::zero);

    let process_mouse_event = move |event: Event<MouseData>| {
        let cursor_coordinates = event.data.element_coordinates();
        log::trace!("cursor_coordinates: {:?}", cursor_coordinates);
        log::trace!(
            "dimensions_min: {:?}, dimensions_max: {:?}, dimensions_size: {:?}",
            dimensions.read().min(),
            dimensions.read().max(),
            dimensions.read().size
        );
        let sv_scale = default::Scale::new(dimensions.read().size.width / 100.0);
        let point_sv = cursor_coordinates.cast_unit() / sv_scale;
        log::trace!("point_sv: {:?}", point_sv);
        color_state.write().saturation = x_axis_to_saturation(point_sv.x);
        color_state.write().brightness = y_axis_to_brightness(point_sv.y);
    };

    render! {
        div {
            class: "saturation-brightness-wrapper",
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
                onmousedown: move |event| {
                    event.stop_propagation();
                    tracking_state.set(true);
                },
                onmouseup: move |event| {
                    event.stop_propagation();
                    tracking_state.set(false);
                },
                onmouseleave: move |event| {
                    event.stop_propagation();
                    tracking_state.set(false);
                },
                onmousemove: move |event| {
                    event.stop_propagation();
                    if *tracking_state.get() {
                        process_mouse_event(event);
                    }
                },
                onclick: move |event| {
                    event.stop_propagation();
                    process_mouse_event(event);
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

#[inline_props]
fn CursorPrimitiveSvg<'a>(
    cx: Scope<'a>,
    x: Option<f64>,
    y: Option<f64>,
    class: Option<&'a str>,
    fill: &'a str,
    transform: Option<&'a str>,
) -> Element<'a> {
    render! {
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
    }
}

fn ColorView(cx: Scope) -> Element {
    let color_state = use_shared_state::<ColorPickerState>(cx).unwrap();
    render! {
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
