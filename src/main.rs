#![allow(non_snake_case)]
use dev_widgets::*;

#[cfg(not(target_family = "wasm"))]
use std::env;

mod logger;

fn main() {
    let mut log_level = log::Level::Warn;
    if cfg!(debug_assertions) {
        #[cfg(not(target_family = "wasm"))]
        env::set_var("RUST_BACKTRACE", "1");
        log_level = log::Level::Trace;
    }

    logger::init(log_level);

    log::info!("Starting app");

    environment::init(App);
}
