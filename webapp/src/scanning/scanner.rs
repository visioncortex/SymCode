use std::u64;

use visioncortex::PointF64;
use wasm_bindgen::prelude::*;

use crate::{canvas::Canvas, util::console_log_util};

use super::{AlphabetReader, AlphabetReaderParams, FinderCandidate, GlyphCode, GlyphLibrary, Recognizer, SymcodeConfig, Transformer, binarize_image_util, is_black_hsv, pipeline::ScanningProcessor, render_binary_image_to_canvas, render_color_image_to_canvas, transform::Transformer as TransformerInterface};

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
        
        let canvas = &Some(Canvas::new_from_id(canvas_id));
        let debug_canvas = &(if !debug_canvas_id.is_empty() {
            Some(Canvas::new_from_id(debug_canvas_id))
        } else {
            None
        });

        let raw_frame = if let Some(canvas) = canvas {
            canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
        } else {
            panic!("Cannot read input image from canvas.");
        };
        
        let binary_raw_frame = binarize_image_util(&raw_frame);

        if let Some(canvas) = canvas {
            render_binary_image_to_canvas(&binary_raw_frame, canvas);
        }
        
        let finder_positions = match FinderCandidate::process(binary_raw_frame, None, canvas) {
            Ok(finder_positions) => finder_positions,
            Err(e) => {
                return e.into();
            }
        };
        
        console_log_util(&format!("Extracted {} finder candidates from raw frame.", finder_positions.len()));
        if finder_positions.len() > max_finder_candidates {
            return "Too many finder candidates!".into();
        }
        
        let symcode_config = &SymcodeConfig {
            code_width: 400,
            code_height: 400,
            symbol_width: 80,
            symbol_height: 80,
            finder_positions: vec![
                PointF64::new(200.0, 80.0),
                PointF64::new(200.0, 200.0),
                PointF64::new(80.0, 320.0),
                PointF64::new(320.0, 320.0)
            ],
            glyph_anchors: vec![
                PointF64::new(40.0, 40.0),
                PointF64::new(40.0, 160.0),
                PointF64::new(160.0, 280.0),
                PointF64::new(280.0, 160.0),
                PointF64::new(280.0, 40.0),
            ],
            rectify_error_threshold: 20.0,
            stat_tolerance: 0.2,
            max_encoding_difference: 1,
            empty_cluster_threshold: 0.2,
            canvas,
            debug_canvas,
        };
        
        let rectified_image = Transformer::transform_image(raw_frame, finder_positions, symcode_config);
        if rectified_image.is_none() {
            return "Cannot rectify image".into();
        }

        let rectified_image = rectified_image.unwrap();

        if let Some(debug_canvas) = debug_canvas {
            match render_color_image_to_canvas(&rectified_image.to_color_image(), debug_canvas) {
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