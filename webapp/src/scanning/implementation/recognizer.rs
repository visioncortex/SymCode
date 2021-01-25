use visioncortex::{ColorImage};

use crate::{math::PerspectiveTransform, scanning::{GlyphReader, SymcodeConfig}};

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

pub struct RecognizerInput<'a> {
    pub raw_frame: ColorImage,
    pub image_to_object: PerspectiveTransform,
    pub glyph_library: &'a GlyphLibrary,
}

impl<'a> Recognizer {

    pub fn process(input: RecognizerInput<'a>, params: &SymcodeConfig) -> Result<Vec<Option<GlyphLabel>>, &'static str> {
        // Processing starts
        let glyph_library = input.glyph_library;
        let glyphs = Self::read_glyphs_from_raw_frame(input.raw_frame, input.image_to_object, glyph_library, params);
        //crate::util::console_log_util(&format!("Recognized glyphs: {:?}", glyphs));
        Ok(glyphs)
    }
}