use wasm_bindgen::prelude::*;

mod canvas;
mod common;
mod crc;
mod math;
mod scanning;
mod util;

#[wasm_bindgen(start)]
pub fn main() {
    util::set_panic_hook();
    console_log::init().unwrap();
}