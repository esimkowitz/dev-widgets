#![allow(non_snake_case)]
use dev_widgets::*;

#[cfg(not(target_family = "wasm"))]
use std::env;

const USE_HOT_RELOAD: bool = false;

mod logger;

fn main() {
    let mut log_level = log::Level::Warn;
    if cfg!(debug_assertions) {
        #[cfg(not(target_family = "wasm"))]
        env::set_var("RUST_BACKTRACE", "1");
        if USE_HOT_RELOAD {
            environment::init_hot_reload();
        }
        log_level = log::Level::Info;
    }

    logger::init(log_level);

    log::info!("Starting app");

    environment::init(App);
}
