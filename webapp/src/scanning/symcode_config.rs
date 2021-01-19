use visioncortex::PointF64;

use crate::canvas::Canvas;

pub struct SymcodeConfig {
    pub code_width: usize,
    pub code_height: usize,

    pub symbol_width: usize,
    pub symbol_height: usize,

    pub finder_positions: Vec<PointF64>,
    /// The top-left corners of the glyphs
    pub glyph_anchors: Vec<PointF64>,

    pub canvas: Option<Canvas>,
    pub debug_canvas: Option<Canvas>,

    pub max_finder_candidates: usize,
    pub rectify_error_threshold: f64,
    pub stat_tolerance: f64,
    pub max_encoding_difference: usize,
    pub empty_cluster_threshold: f64,
}