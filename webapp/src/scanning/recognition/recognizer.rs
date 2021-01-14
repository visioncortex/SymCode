use visioncortex::ColorImage;

use crate::{canvas::Canvas, scanning::color_image_to_merged_clusters};

use super::{GlyphCode, GlyphLibrary};

/// Takes a rectified code image (assumed to be valid), recognizes the glyphs on it
pub struct Recognizer {}

impl Recognizer {
    pub fn recognize_glyphs_on_image(image: ColorImage, anchor_error_threshold: f64, glyph_library: &GlyphLibrary, stat_tolerance: f64, debug_canvas: &Option<Canvas>) -> GlyphCode {
        let images_rects = color_image_to_merged_clusters(image, 10, 10); // Expand each cluster by 10 units vertically and horizontally
        
        let mut glyph_code = GlyphCode::default();
        glyph_code.add_images_rects_near_anchors(images_rects, anchor_error_threshold, glyph_library, stat_tolerance, debug_canvas);

        glyph_code
    }
}