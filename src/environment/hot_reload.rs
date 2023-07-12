use dioxus::prelude::*;
use dioxus_hot_reload::{hot_reload_init, Config as HotReloadConfig};

pub fn init_hot_reload() {
    hot_reload_init!(HotReloadConfig::new()
        .with_paths(&["src", "style", "scss"])
        .with_rebuild_command("cargo run"));
}
