use dioxus::dioxus_core::Element;

#[cfg(not(target_family = "wasm"))]
mod desktop;

#[cfg(target_family = "wasm")]
mod web;

pub fn init(root: fn() -> Element) {
    #[cfg(not(target_family = "wasm"))]
    desktop::init_app(root);

    #[cfg(target_family = "wasm")]
    web::init_app(root);
}
