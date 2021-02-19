use bit_vec::BitVec;

use crate::acute32::Decoder;

pub struct Acute32Decoder;

// Dummy implementation, error detection/correction will be supported later
impl Decoder for Acute32Decoder {
    type Symbol = Option<super::glyph::GlyphLabel>;
    // To Sanford: why option here? dont we have GlyphLabel::Invalid already? Shall we add an GlyphLabel::None?

    type Err = &'static str;

    fn decode(encoded_data: Vec<Self::Symbol>, num_templates: usize) -> Result<BitVec, Self::Err> {
        let num_bits_per_glyph = crate::math::num_bits(num_templates);

        let mut decoded_data = vec![];
        for datum in encoded_data.iter() {
            if *datum == Some(super::glyph::GlyphLabel::Invalid) {
                return Err("Some recognized glyph is invalid.");
            }
            if let Some(bit_vec) = super::glyph::GlyphLabel::option_self_to_bit_vec(*datum, num_bits_per_glyph) {
                decoded_data.push(bit_vec);
            } else {
                panic!("Invalid glyphs fed into bit vec conversion.");
            }
        }
        
        let total_bits = num_bits_per_glyph * decoded_data.len();
        Ok(
            BitVec::from_fn(total_bits, |i| {
                let glyph_index = i / num_bits_per_glyph;
                let within_glyph_offset = i % num_bits_per_glyph;
                decoded_data[glyph_index].get(within_glyph_offset).unwrap()
            })
        )
    }
}

impl Acute32Decoder {
    pub fn process(input: Vec<Option<super::glyph::GlyphLabel>>) -> Result<BitVec, &'static str> {
        Self::decode(input, super::glyph::GlyphLabel::num_variants())
    }
}