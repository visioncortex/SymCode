use std::u64;

use visioncortex::{ColorImage};

use crate::{canvas::Canvas, scanning::{binarize_image}};

use super::{GlyphCode, GlyphLibrary};

/// Takes a rectified code image (assumed to be valid), recognizes the glyphs on it
pub struct Recognizer {}

impl Recognizer {
    pub fn recognize_glyphs_on_image(image: ColorImage,
            glyph_library: &GlyphLibrary,
            stat_tolerance: f64, max_encoding_difference: usize, empty_cluster_threshold: u64,
            debug_canvas: &Option<Canvas>) -> GlyphCode {
        let image = binarize_image(&image);
        GlyphCode::from_rectified_image_by_cropping(
            image,
            GlyphCode::GLYPH_SIZE,
            glyph_library,
            stat_tolerance,
            max_encoding_difference,
            empty_cluster_threshold,
            debug_canvas
        )
    }
}