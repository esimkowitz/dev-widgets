#![allow(non_snake_case)]
use dev_widgets::*;
use std::env;

const USE_HOT_RELOAD: bool = false;

fn main() {
    if cfg!(debug_assertions) {
        #[cfg(not(target_family = "wasm"))]
        env::set_var("RUST_BACKTRACE", "1");
        if USE_HOT_RELOAD {
            environment::init_hot_reload();
        }
    }

    environment::init(App);
}
