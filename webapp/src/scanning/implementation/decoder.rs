use crate::scanning::Decoder;

pub struct SymcodeDecoder;

// Dummy implementation, error detection/correction will be supported later
impl Decoder for SymcodeDecoder {
    type EncodedData = Vec<Option<super::glyph::GlyphLabel>>;

    type DecodedData = Vec<Option<super::glyph::GlyphLabel>>;

    type DecodeError = String;

    fn decode(encoded_data: Self::EncodedData) -> Result<Self::DecodedData, Self::DecodeError> {
        Ok(encoded_data)
    }
}