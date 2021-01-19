use crate::scanning::{GlyphReader};

use super::{GlyphLabel, GlyphLibrary};

/// An implementation of GlyphReader
pub struct Recognizer;

impl GlyphReader for Recognizer {
    type Label = GlyphLabel;

    type Library = GlyphLibrary;

    fn find_most_similar_glyph(image: visioncortex::BinaryImage, glyph_library: &Self::Library, symcode_config: &crate::scanning::SymcodeConfig) -> Self::Label {
        glyph_library.find_most_similar_glyph(
            image,
            symcode_config
        )
    }
}