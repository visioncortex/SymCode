use wasm_bindgen::prelude::*;

mod canvas;
mod common;
mod crc;
mod generator;
mod math;
mod scanner;
mod scanning;
mod symbol;
mod util;

#[wasm_bindgen(start)]
pub fn main() {
    util::set_panic_hook();
    console_log::init().unwrap();
}