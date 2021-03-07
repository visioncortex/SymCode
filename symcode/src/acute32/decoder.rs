use bit_vec::BitVec;
use crate::interfaces::Decoder;
use super::{Acute32SymcodeConfig, GlyphLabel};

pub struct Acute32Decoder<'a> {
    #[allow(dead_code)]
    config: &'a Acute32SymcodeConfig,
}

impl<'a> Acute32Decoder<'a> {
    pub fn new(config: &'a Acute32SymcodeConfig) -> Acute32Decoder<'a> {
        Self { config }
    }
}

// Dummy implementation, error detection/correction will be supported later
impl Decoder for Acute32Decoder<'_> {
    type Symbol = GlyphLabel;

    type Err = &'static str;

    fn decode(&self, encoded_data: Vec<Self::Symbol>) -> Result<BitVec, Self::Err> {
        let num_bits_per_symbol = self.num_bits_per_symbol();
        let mut decoded_data = vec![];
        for &symbol in encoded_data.iter() {
            if let Some(bit_vec) = GlyphLabel::self_to_bit_vec(symbol, num_bits_per_symbol) {
                decoded_data.push(bit_vec);
            } else {
                return Err("Decoder error: Some recognized glyph is invalid.");
            }
        }

        // Extract the first 20 bits as data payload, and the rest (5 bits) as checksum
        let mut payload = BitVec::from_elem(20, false);
        let mut checksum: u8 = 0;
        for i in 0..25 {
            let glyph_index = i / num_bits_per_symbol;
            let within_glyph_offset = i % num_bits_per_symbol;
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

    fn num_bits_per_symbol(&self) -> usize {
        crate::math::num_bits_to_store(GlyphLabel::num_variants())
    }
}
