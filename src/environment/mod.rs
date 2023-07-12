use dioxus::prelude::Component;

#[cfg(not(target_family = "wasm"))]
mod desktop;

#[cfg(target_family = "wasm")]
mod web;

#[cfg(all(not(target_family = "wasm"), debug_assertions))]
mod hot_reload;

pub fn init(root: Component) {
    #[cfg(not(target_family = "wasm"))]
    desktop::init_app(root);

    #[cfg(target_family = "wasm")]
    web::init_app(root);
}

pub fn init_hot_reload() {
    #[cfg(all(not(target_family = "wasm"), debug_assertions))]
    hot_reload::init_hot_reload();
}