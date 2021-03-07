use wasm_bindgen::prelude::*;

pub mod acute32;
pub mod interfaces;
pub mod app;
pub mod canvas;
pub mod common;
pub mod debugger;
pub mod math;
pub mod util;

#[wasm_bindgen(start)]
pub fn main() {
    util::set_panic_hook();
    console_log::init().unwrap();
}