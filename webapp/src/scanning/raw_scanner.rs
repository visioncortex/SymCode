use visioncortex::BinaryImage;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::{canvas::Canvas, utils::render_color_image_to_canvas};

use super::{FinderCandidate, GlyphLabel, GlyphLibrary, Recognizer, is_black, transform::Transformer};

#[wasm_bindgen]
pub struct RawScanner {
    glyph_library: GlyphLibrary,
}

impl Default for RawScanner {
    fn default() -> Self {
        Self { glyph_library: GlyphLibrary::default() }
    }
}

#[wasm_bindgen]
impl RawScanner {
    pub fn new() -> Self {
        Self::default()
    }

    /// Takes the id of the canvas element storing the template image, and the usize representation of the glyph label
    pub fn load_template_from_canvas_id(&mut self, canvas_id: &str, label: usize) {
        let canvas = &Canvas::new_from_id(canvas_id);
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black(&c.to_hsv()));
        self.glyph_library.add_template(image, label);
    }

    /// Initiate scanning, should return whatever info is needed for decoding
    pub fn scan_from_canvas_id(&self, canvas_id: &str, debug_canvas_id: &str, rectify_error_threshold: f64, anchor_error_threshold: f64) -> JsValue {
        let canvas = &Canvas::new_from_id(canvas_id);
        let debug_canvas = &Canvas::new_from_id(debug_canvas_id);

        let raw_frame = canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32);
        let finder_candidates = FinderCandidate::extract_finder_candidates(
            &raw_frame,
            canvas,
            debug_canvas
        );
        if let Some(rectified_image) = Transformer::rectify_image(raw_frame, finder_candidates, rectify_error_threshold) {
            match render_color_image_to_canvas(&rectified_image, debug_canvas) {
                Ok(_) => {},
                Err(e) => {return e},
            }

            let glyph_code = Recognizer::recognize_glyphs_on_image(rectified_image, anchor_error_threshold, &self.glyph_library, debug_canvas);
            
            console::log_1(&format!("{:?}", glyph_code).into());
            
            "Recognition complete".into()
        } else {
            "Cannot rectify image".into()
        }
    }
}