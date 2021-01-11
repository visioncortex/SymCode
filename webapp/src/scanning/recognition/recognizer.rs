use visioncortex::ColorImage;
use web_sys::console;

use crate::{canvas::Canvas, scanning::color_image_to_merged_clusters};

use super::{GlyphCode, GlyphLibrary};

/// Takes a rectified code image (assumed to be valid), recognizes the glyphs on it
pub struct Recognizer {}

impl Recognizer {
    pub fn recognize_glyphs_on_image(image: ColorImage, anchor_error_threshold: f64, glyph_library: &GlyphLibrary, debug_canvas: &Canvas) -> GlyphCode {
        let images_rects = color_image_to_merged_clusters(image, 7, 7); // Expand each cluster by 7 units vertically and horizontally
        
        let mut glyph_code = GlyphCode::default();
        glyph_code.add_clusters_near_anchors(images_rects, anchor_error_threshold, glyph_library , debug_canvas);

        glyph_code
    }
}