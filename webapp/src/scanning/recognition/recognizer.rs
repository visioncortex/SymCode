use visioncortex::ColorImage;

use crate::scanning::color_image_to_clusters;

use super::GlyphCode;

/// Takes a rectified code image (assumed to be valid), recognizes the glyphs on it
pub struct Recognizer {}

impl Recognizer {
    pub fn recognize_glyphs_on_image(image: ColorImage) -> GlyphCode {
        let clusters = color_image_to_clusters(image);

        let mut glyph_code = GlyphCode::default();
        glyph_code.add_clusters_to_anchors(clusters);

        GlyphCode::default()
    }
}