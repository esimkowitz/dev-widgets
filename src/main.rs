#![allow(non_snake_case)]
use dev_widgets::*;
use dioxus::logger::tracing::Level;

#[cfg(not(target_family = "wasm"))]
use std::env;

fn main() {
    let log_level = if cfg!(debug_assertions) {
        #[cfg(not(target_family = "wasm"))]
        env::set_var("RUST_BACKTRACE", "1");
        Level::INFO
    } else {
        Level::WARN
    };

    dioxus::logger::init(log_level).expect("logger failed to init");
    tracing_log::LogTracer::init().expect("log-to-tracing bridge failed to init");

    tracing::info!("Starting app");

    environment::init(App);
}
