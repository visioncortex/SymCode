use crate::scanning::Decoder;

pub struct SymcodeDecoder;

// Dummy implementation, error detection/correction will be supported later
impl Decoder for SymcodeDecoder {
    type EncodedData = Vec<Option<super::glyph::GlyphLabel>>;

    type DecodedData = Vec<Option<super::glyph::GlyphLabel>>;

    type Err = &'static str;

    fn decode(encoded_data: Self::EncodedData) -> Result<Self::DecodedData, Self::Err> {
        encoded_data.iter().for_each(|datum| {
            crate::util::console_log_util(super::glyph::GlyphLabel::option_self_to_primitive(*datum));
        });
        Ok(encoded_data)
    }
}

impl SymcodeDecoder {
    pub fn process(input: Vec<Option<super::glyph::GlyphLabel>>) -> Result<Vec<Option<super::glyph::GlyphLabel>>, &'static str> {
        Self::decode(input)
    }
}