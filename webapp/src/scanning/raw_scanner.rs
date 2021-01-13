use visioncortex::PointI32;
use wasm_bindgen::prelude::*;


use crate::{canvas::Canvas, util::console_log_util};

use super::{AlphabetReader, AlphabetReaderParams, FinderCandidate, GlyphCode, GlyphLibrary, Recognizer, is_black, render_color_image_to_canvas, transform::Transformer};

#[wasm_bindgen]
pub struct RawScanner {
    glyph_library: GlyphLibrary,
    stat_tolerance: f64,
}

impl Default for RawScanner {
    fn default() -> Self {
        Self {
            glyph_library: GlyphLibrary::default(),
            stat_tolerance: 0.2,
        }
    }
}

#[wasm_bindgen]
impl RawScanner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_stat_tolerance(stat_tolerance: f64) -> Self {
        Self {
            glyph_library: GlyphLibrary::default(),
            stat_tolerance,
        }
    }

    /// Takes the id of the canvas element storing the template image, and the usize representation of the glyph label
    pub fn load_template_from_canvas_id(&mut self, canvas_id: &str) {
        let canvas = &Canvas::new_from_id(canvas_id);
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black(&c.to_hsv()));
        self.glyph_library.add_template(image, self.stat_tolerance);
    }

    /// Takes the id of the canvas element storing the alphabet. The parameters are currently hardcoded here.
    pub fn load_alphabet_from_canvas_id(&mut self, canvas_id: &str) {
        let canvas = &Canvas::new_from_id(canvas_id);
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black(&c.to_hsv()));
        let params = AlphabetReaderParams {
            top_left: PointI32::new(53, 53),
            glyph_width: GlyphCode::GLYPH_SIZE,
            glyph_height: GlyphCode::GLYPH_SIZE,
            offset_x: 111,
            offset_y: 112,
            num_columns: 4,
            num_rows: 4,
        };
        AlphabetReader::read_alphabet_to_library(&mut self.glyph_library, image, params, self.stat_tolerance);
    }

    /// Initiate scanning, should return whatever info is needed for decoding
    pub fn scan_from_canvas_id(&self, canvas_id: &str, debug_canvas_id: &str, rectify_error_threshold: f64, anchor_error_threshold: f64, finder_candidates_upper_limit: usize) -> JsValue {
        if self.glyph_library.is_empty() {
            return "No templates loaded into RawScanner object yet!".into();
        }
        
        let canvas = &Canvas::new_from_id(canvas_id);
        let debug_canvas = &Canvas::new_from_id(debug_canvas_id);

        let raw_frame = canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32);
        let finder_candidates = FinderCandidate::extract_finder_candidates(
            &raw_frame,
            canvas,
            debug_canvas
        );
        console_log_util(&format!("Extracted {} finder candidates from raw frame.", finder_candidates.len()));
        if finder_candidates.len() > finder_candidates_upper_limit {
            return "Too many finder candidates!".into();
        }
        
        if let Some(rectified_image) = Transformer::rectify_image(raw_frame, finder_candidates, rectify_error_threshold) {
            match render_color_image_to_canvas(&rectified_image, debug_canvas) {
                Ok(_) => {},
                Err(e) => {return e},
            }

            let glyph_code = Recognizer::recognize_glyphs_on_image(rectified_image, anchor_error_threshold, &self.glyph_library, self.stat_tolerance, debug_canvas);
            
            console_log_util(&format!("{:?}", glyph_code));
            
            "Recognition complete".into()
        } else {
            "Cannot rectify image".into()
        }
    }
}