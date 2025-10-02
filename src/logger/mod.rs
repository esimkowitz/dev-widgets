#[cfg(not(target_family = "wasm"))]
use log::*;

#[cfg(not(target_family = "wasm"))]
mod simple_logger;

pub fn init(level: log::Level) {
    #[cfg(target_family = "wasm")]
    {
        wasm_logger::init(wasm_logger::Config::new(level));
    }

    #[cfg(not(target_family = "wasm"))]
    {
        match set_boxed_logger(Box::new(simple_logger::SimpleLogger)) {
            Ok(_) => log::set_max_level(level.to_level_filter()),
            Err(e) => panic!("Failed to initialize logger: {}", e),
        }
    }
}
