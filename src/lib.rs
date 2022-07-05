mod utils;
use wasm_bindgen::prelude::*;
use web_sys::{console::log_1 as log, window};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() {
    log(&JsValue::from(format!("{} {}", "Greet", "World")));
    window().unwrap().alert_with_message("Greet from Rust!");
}

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();
}
