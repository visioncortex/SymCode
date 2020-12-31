use wasm_bindgen::prelude::*;

mod canvas;
mod common;
//mod math;
mod scanning;
mod utils;

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
    console_log::init().unwrap();
}