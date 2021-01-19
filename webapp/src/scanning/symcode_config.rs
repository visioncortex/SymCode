use visioncortex::PointF64;
use wasm_bindgen::prelude::*;

use crate::canvas::Canvas;

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

    pub(crate) canvas: Option<Canvas>,
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
            canvas: Canvas::new_from_id("frame"),
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
    pub fn max_finder_candidates(&self) -> usize {
        self.finder_positions.len() + self.max_extra_finder_candidates
    }
}

#[wasm_bindgen]
impl SymcodeConfig {
    pub fn new() -> Self {
        Self::default()
    }
}