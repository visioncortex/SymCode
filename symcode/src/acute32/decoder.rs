use bit_vec::BitVec;

use crate::interfaces::Decoder;

use super::GlyphLabel;

pub struct Acute32Decoder;

// Dummy implementation, error detection/correction will be supported later
impl Decoder for Acute32Decoder {
    type Symbol = GlyphLabel;

    type Err = &'static str;

    fn decode(encoded_data: Vec<Self::Symbol>, num_templates: usize) -> Result<BitVec, Self::Err> {
        let num_bits_per_glyph = crate::math::num_bits_to_store(num_templates);
        let mut decoded_data = vec![];
        for &symbol in encoded_data.iter() {
            if let Some(bit_vec) = GlyphLabel::self_to_bit_vec(symbol, num_bits_per_glyph) {
                decoded_data.push(bit_vec);
            } else {
                return Err("Decoder error: Some recognized glyph is invalid.");
            }
        }

        // Extract the first 20 bits as data payload, and the rest (5 bits) as checksum
        let mut payload = BitVec::from_elem(20, false);
        let mut checksum: u8 = 0;
        for i in 0..25 {
            let glyph_index = i / num_bits_per_glyph;
            let within_glyph_offset = i % num_bits_per_glyph;
            let bit = decoded_data[glyph_index].get(within_glyph_offset).unwrap();
            if i < 20 {
                payload.set(i, bit);
            } else {
                checksum <<= 1;
                checksum += if bit {1} else {0};
            }
        }

        if crczoo::crc5(&payload.to_bytes()) != checksum {
            Err("Decoder error: Checksum fail")
        } else {
            Ok(payload)
        }
    }
}

impl Acute32Decoder {
    pub fn process(input: Vec<GlyphLabel>) -> Result<BitVec, &'static str> {
        Self::decode(input, GlyphLabel::num_variants())
    }
}