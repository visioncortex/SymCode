use std::u64;

use visioncortex::{ColorImage, ColorSum, PointI32};
use wasm_bindgen::prelude::*;

use crate::{canvas::Canvas, util::console_log_util};

use super::{AlphabetReader, AlphabetReaderParams, FinderCandidate, GlyphCode, GlyphLibrary, Recognizer, binarize_image, is_black_hsv, render_color_image_to_canvas, transform::Transformer};

#[wasm_bindgen]
pub struct SymcodeScanner {
    glyph_library: GlyphLibrary,
    stat_tolerance: f64,
}

impl Default for SymcodeScanner {
    fn default() -> Self {
        Self {
            glyph_library: GlyphLibrary::default(),
            stat_tolerance: 0.2,
        }
    }
}

#[wasm_bindgen]
impl SymcodeScanner {
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
            .to_binary_image(|c| is_black_hsv(&c.to_hsv()));
        self.glyph_library.add_template(image, self.stat_tolerance);
    }

    /// Takes the id of the canvas element storing the alphabet.
    pub fn load_alphabet_from_canvas_id(&mut self, canvas_id: &str, params: AlphabetReaderParams) {
        let canvas = &Canvas::new_from_id(canvas_id);
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black_hsv(&c.to_hsv()));
        AlphabetReader::read_alphabet_to_library(&mut self.glyph_library, image, params, self.stat_tolerance);
    }

    pub fn scan_with_config(&self, config: SymcodeScannerConfig) -> JsValue {
        Self::scan_from_canvas_id(
            self,
            &config.canvas_id,
            &config.debug_canvas_id,
            config.rectify_error_threshold,
            config.max_finder_candidates,
            config.max_encoding_difference,
            config.empty_cluster_threshold,
        )
    }

    /// Initiate scanning, should return whatever info is needed for decoding
    #[allow(clippy::too_many_arguments)]
    pub fn scan_from_canvas_id(&self, canvas_id: &str, debug_canvas_id: &str,
        rectify_error_threshold: f64, max_finder_candidates: usize,
        max_encoding_difference: usize, empty_cluster_threshold: f64
    ) -> JsValue {
        if self.glyph_library.is_empty() {
            return "No templates loaded into RawScanner object yet!".into();
        }
        
        let canvas = &Canvas::new_from_id(canvas_id);
        let debug_canvas = &(if !debug_canvas_id.is_empty() {
            Some(Canvas::new_from_id(debug_canvas_id))
        } else {
            None
        });

        let raw_frame = canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32);
        
        let binary_raw_frame = binarize_image(&raw_frame);
        
        let finder_candidates = FinderCandidate::extract_finder_candidates(
            binary_raw_frame,
            canvas,
            debug_canvas
        );
        console_log_util(&format!("Extracted {} finder candidates from raw frame.", finder_candidates.len()));
        if finder_candidates.len() > max_finder_candidates {
            return "Too many finder candidates!".into();
        }
        
        if let Some(rectified_image) = Transformer::rectify_image(raw_frame, finder_candidates, rectify_error_threshold) {
            if let Some(debug_canvas) = debug_canvas {
                match render_color_image_to_canvas(&rectified_image, debug_canvas) {
                    Ok(_) => {},
                    Err(e) => {return e},
                }
            }

            let glyph_code = Recognizer::recognize_glyphs_on_image(
                rectified_image,
                &self.glyph_library,
                self.stat_tolerance,
                max_encoding_difference,
                (empty_cluster_threshold * (GlyphCode::GLYPH_SIZE * GlyphCode::GLYPH_SIZE) as f64) as u64,
                debug_canvas);
            
            console_log_util(&format!("{:?}", glyph_code));
            
            "Success".into()
        } else {
            "Cannot rectify image".into()
        }
    }
}

impl SymcodeScanner {
    /// Downsample by shifting the width/height by the specified amount.
    ///
    /// Prefer cropping original frame over downsampling.
    fn downsample_raw_frame(raw_frame: ColorImage, shift_bits: usize) -> ColorImage {
        let new_width = raw_frame.width >> shift_bits;
        let new_height = raw_frame.height >> shift_bits;
        let mut result = ColorImage::new_w_h(new_width, new_height);
        let step: usize = 1 << shift_bits; // step size in sampling

        let get_average_color_in_square = |top_left: PointI32| {
            let mut color_sum = ColorSum::new();

            for i in 0..step {
                for j in 0..step {
                    let point = top_left + PointI32::new(j as i32, i as i32);
                    if let Some(color) = raw_frame.get_pixel_at_point_safe(point) {
                        color_sum.add(&color);
                    }
                }
            }

            color_sum.average()
        };

        for i in 0..(raw_frame.height/step) {
            for j in 0..(raw_frame.width/step) {
                let average_color = get_average_color_in_square(PointI32::new((j*step) as i32, (i*step) as i32));
                //console_log_util(&format!("{} {}", j, i));
                result.set_pixel(j/step, i/step, &average_color);
            }
        }

        result
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct SymcodeScannerConfig {
    canvas_id: String,
    debug_canvas_id: String,
    rectify_error_threshold: f64,
    max_finder_candidates: usize,
    max_encoding_difference: usize,
    empty_cluster_threshold: f64,
}

impl Default for SymcodeScannerConfig {
    fn default() -> Self {
        Self {
            canvas_id: "frame".into(),
            debug_canvas_id: "".into(),
            rectify_error_threshold: 20.0,
            max_finder_candidates: 7,
            max_encoding_difference: 1,
            empty_cluster_threshold: 0.2,
        }
    }
}

#[wasm_bindgen]
impl SymcodeScannerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_canvas_id(canvas_id: &str) -> Self {
        Self::default()
            .canvas(canvas_id)
    }

    // Can't use macros inside wasm_bindgen impls

    pub fn canvas(mut self, canvas_id: &str) -> Self {
        self.canvas_id = canvas_id.into();
        self
    }

    pub fn debug_canvas(mut self, debug_canvas_id: &str) -> Self {
        self.debug_canvas_id = debug_canvas_id.into();
        self
    }

    pub fn rectify_error_threshold(mut self, rectify_error_threshold: f64) -> Self {
        self.rectify_error_threshold = rectify_error_threshold;
        self
    }

    pub fn max_finder_candidates(mut self, max_finder_candidates: usize) -> Self {
        self.max_finder_candidates = max_finder_candidates;
        self
    }

    pub fn max_encoding_difference(mut self, max_encoding_difference: usize) -> Self {
        self.max_encoding_difference = max_encoding_difference;
        self
    }
}