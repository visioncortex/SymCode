use visioncortex::{ColorImage};

use crate::{math::PerspectiveTransform, acute32::{GlyphReader, Acute32SymcodeConfig}};

use super::{GlyphLabel, Acute32Library};

/// An implementation of GlyphReader
pub struct Acute32Recognizer;

impl GlyphReader for Acute32Recognizer {
    type Label = GlyphLabel;

    type Library = Acute32Library;

    fn find_most_similar_glyph(image: visioncortex::BinaryImage, glyph_library: &Self::Library, symcode_config: &crate::acute32::Acute32SymcodeConfig) -> Self::Label {
        glyph_library.find_most_similar_glyph(
            image,
            symcode_config
        )
    }
}

pub struct RecognizerInput<'a> {
    pub raw_frame: ColorImage,
    pub image_to_object: PerspectiveTransform,
    pub glyph_library: &'a Acute32Library,
}

impl<'a> Acute32Recognizer {

    pub fn process(input: RecognizerInput<'a>, params: &Acute32SymcodeConfig) -> Result<Vec<Option<GlyphLabel>>, &'static str> {
        // Processing starts
        let glyph_library = input.glyph_library;
        let glyphs = Self::read_glyphs_from_raw_frame(input.raw_frame, input.image_to_object, glyph_library, params);
        //crate::util::console_log_util(&format!("Recognized glyphs: {:?}", glyphs));
        Ok(glyphs)
    }
}