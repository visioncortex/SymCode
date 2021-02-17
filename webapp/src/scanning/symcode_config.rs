use visioncortex::{BinaryImage, PointF64};
use wasm_bindgen::prelude::*;

use crate::canvas::Canvas;

use super::valid_pointf64_on_image;

#[wasm_bindgen]
pub struct SymcodeConfig {
    pub code_width: usize,
    pub code_height: usize,

    pub symbol_width: usize,
    pub symbol_height: usize,

    /// The centers of the finders
    pub(crate) finder_positions: Vec<PointF64>,
    /// The top-left corners of the glyphs
    pub(crate) glyph_anchors: Vec<PointF64>,

    pub(crate) debug_canvas: Option<Canvas>,

    pub max_extra_finder_candidates: usize,
    pub rectify_error_threshold: f64,
    pub stat_tolerance: f64,
    pub max_encoding_difference: usize,
    pub empty_cluster_threshold: f64,
}

impl Default for SymcodeConfig {
    fn default() -> Self {
        Self {
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
            debug_canvas: None,
            max_extra_finder_candidates: 3,
            rectify_error_threshold: 20.0,
            stat_tolerance: 0.2,
            max_encoding_difference: 1,
            empty_cluster_threshold: 0.2,
        }
    }
}

impl SymcodeConfig {
    #[inline]
    pub fn max_finder_candidates(&self) -> usize {
        self.finder_positions.len() + self.max_extra_finder_candidates
    }

    #[inline]
    pub fn absolute_empty_cluster_threshold(&self, image_width: usize, image_height: usize) -> u64 {
        (self.empty_cluster_threshold * (image_width * image_height) as f64) as u64
    }

    #[inline]
    pub fn num_glyphs_in_code(&self) -> usize {
        self.glyph_anchors.len()
    }
}

#[wasm_bindgen]
impl SymcodeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    // Can't use macros inside wasm_bindgen impls

    pub fn debug_canvas(mut self, debug_canvas_id: &str) -> Self {
        self.debug_canvas = Canvas::new_from_id(debug_canvas_id);
        self
    }

    pub fn code_width(mut self, code_width: usize) -> Self {
        self.code_width = code_width;
        self
    }

    pub fn code_height(mut self, code_height: usize) -> Self {
        self.code_height = code_height;
        self
    }

    pub fn symbol_width(mut self, symbol_width: usize) -> Self {
        self.symbol_width = symbol_width;
        self
    }

    pub fn symbol_height(mut self, symbol_height: usize) -> Self {
        self.symbol_height = symbol_height;
        self
    }

    pub fn max_extra_finder_candidates(mut self, max_extra_finder_candidates: usize) -> Self {
        self.max_extra_finder_candidates = max_extra_finder_candidates;
        self
    }

    pub fn rectify_error_threshold(mut self, rectify_error_threshold: f64) -> Self {
        self.rectify_error_threshold = rectify_error_threshold;
        self
    }

    pub fn stat_tolerance(mut self, stat_tolerance: f64) -> Self {
        self.stat_tolerance = stat_tolerance;
        self
    }

    pub fn max_encoding_difference(mut self, max_encoding_difference: usize) -> Self {
        self.max_encoding_difference = max_encoding_difference;
        self
    }

    pub fn empty_cluster_threshold(mut self, empty_cluster_threshold: f64) -> Self {
        self.empty_cluster_threshold = empty_cluster_threshold;
        self
    }
}

#[wasm_bindgen]
impl SymcodeConfig {
    pub fn add_finder_position(&mut self, x: f64, y: f64) -> String {
        let finder_position = PointF64::new(x, y);
        if valid_pointf64_on_image(finder_position, self.code_width, self.code_height) {
            self.finder_positions.push(finder_position);
            format!("Finder ({}, {}) added.", x, y)
        } else {
            format!("Finder ({}, {}) is not within the boundary of the code.", x, y)
        }
    }

    pub fn add_glyph_anchor(&mut self, x: f64, y: f64) -> String {
        let glyph_anchor = PointF64::new(x, y);
        if valid_pointf64_on_image(glyph_anchor, self.code_width, self.code_height) {
            self.glyph_anchors.push(glyph_anchor);
            format!("Glyph anchor ({}, {}) added.", x, y)
        } else {
            format!("Glyph anchor ({}, {}) is not within the boundary of the code.", x, y)
        }
    }
}

#[wasm_bindgen]
impl SymcodeConfig {
    pub fn from_json_string(json_string: &str) -> Self {
        let json: serde_json::Value = serde_json::from_str(json_string).unwrap();

        let finder_positions: Vec<PointF64> = json["finder_positions"].as_array().unwrap().iter().map(|p| {
            PointF64::new(p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap())
        }).collect();

        let glyph_anchors: Vec<PointF64> = json["glyph_anchors"].as_array().unwrap().iter().map(|p| {
            PointF64::new(p["x"].as_f64().unwrap(), p["y"].as_f64().unwrap())
        }).collect();

        Self {
            code_width: json["code_width"].as_i64().unwrap() as usize,
            code_height: json["code_height"].as_i64().unwrap() as usize,
            symbol_width: json["symbol_width"].as_i64().unwrap() as usize,
            symbol_height: json["symbol_height"].as_i64().unwrap() as usize,
            finder_positions,
            glyph_anchors,
            debug_canvas: Canvas::new_from_id(json["debug_canvas"].as_str().unwrap()),
            max_extra_finder_candidates: json["max_extra_finder_candidates"].as_i64().unwrap() as usize,
            rectify_error_threshold: json["rectify_error_threshold"].as_f64().unwrap(),
            stat_tolerance: json["stat_tolerance"].as_f64().unwrap(),
            max_encoding_difference: json["max_encoding_difference"].as_i64().unwrap() as usize,
            empty_cluster_threshold: json["empty_cluster_threshold"].as_f64().unwrap(),
        }
    }
}