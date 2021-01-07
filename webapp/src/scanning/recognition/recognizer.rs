use visioncortex::ColorImage;

use crate::{canvas::Canvas, scanning::color_image_to_clusters};

use super::{GlyphCode, GlyphLibrary};

/// Takes a rectified code image (assumed to be valid), recognizes the glyphs on it
pub struct Recognizer {}

impl Recognizer {
    pub fn recognize_glyphs_on_image(image: ColorImage, anchor_error_threshold: f64, glyph_library: &GlyphLibrary, debug_canvas: &Canvas) -> GlyphCode {
        let clusters = color_image_to_clusters(image);

        let mut glyph_code = GlyphCode::default();
        glyph_code.add_clusters_near_anchors(clusters, anchor_error_threshold, glyph_library , debug_canvas);

        glyph_code
    }
}