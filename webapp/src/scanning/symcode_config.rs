use visioncortex::PointF64;

pub struct SymcodeConfig {
    pub code_width: usize,
    pub code_height: usize,

    pub glyph_width: usize,
    pub glyph_height: usize,

    pub finder_positions: Vec<PointF64>,

    pub rectify_error_threshold: f64,
}