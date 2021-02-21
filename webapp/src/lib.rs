use wasm_bindgen::prelude::*;

mod canvas;
mod common;
mod interfaces;
mod math;
mod acute32;
mod util;

#[wasm_bindgen(start)]
pub fn main() {
    util::set_panic_hook();
    console_log::init().unwrap();
}