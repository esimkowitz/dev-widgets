use dioxus::prelude::Component;

#[cfg(not(target_family = "wasm"))]
mod desktop;

#[cfg(all(debug_assertions, not(target_family = "wasm")))]
mod hot_reload;

pub fn init(root: Component) {
    #[cfg(not(target_family = "wasm"))]
    desktop::init_app(root);
}

pub fn init_hot_reload() {
    #[cfg(all(debug_assertions, not(target_family = "wasm")))]
    hot_reload::init_hot_reload();
}