use bit_vec::BitVec;

use crate::interfaces::{Decoder as DecoderInterface, Encoder as EncoderInterface};

use super::{Acute32SymcodeConfig, Acute32Decoder, GlyphLabel};

pub struct Acute32Encoder<'a> {
    config: &'a Acute32SymcodeConfig,
}

impl<'a> Acute32Encoder<'a> {
    pub fn new(config: &'a Acute32SymcodeConfig) -> Acute32Encoder<'a> {
        Self { config }
    }
}

impl EncoderInterface for Acute32Encoder<'_> {
    type SymcodeRepresentation = Vec<GlyphLabel>;

    fn encode(&self, payload: bit_vec::BitVec, num_glyphs: usize) -> Result<Self::SymcodeRepresentation, &'static str> {
        let symbol_num_bits = crate::math::num_bits_to_store(GlyphLabel::num_variants());
        if payload.len() != (symbol_num_bits*num_glyphs - 5) { // Reserve 5 bits for CRC5 checksum
            panic!("Input bits length and self-defined length do not agree!");
        }
        
        let checksum = crate::math::into_bitvec(crczoo::crc5(&payload.to_bytes()) as usize, 5);
        
        // This payload is used to generate the code image
        let payload_with_checksum = BitVec::from_fn(
            payload.len() + checksum.len(),
            |i| {
                // Concatenate the data and checksum
                if i < payload.len() {
                    payload.get(i).unwrap()
                } else {
                    checksum.get(i - payload.len()).unwrap()
                }
            }
        );

        // Artificial data corruption
        //payload_with_checksum.set(5, !payload_with_checksum.get(5).unwrap());

        let mut result: Self::SymcodeRepresentation = Vec::with_capacity(num_glyphs);

        for i in 0..num_glyphs {
            let symbol_bit_vec = BitVec::from_fn(symbol_num_bits, |j| { // j is in [0, symbol_num_bits-1]
                let index = i*symbol_num_bits + j;
                payload_with_checksum[index]
            });
            result.push(GlyphLabel::from_bit_vec(symbol_bit_vec));
        }

        // Sanity check
        match Acute32Decoder::new(self.config).decode(result.clone()) {
            Ok(decoded_payload) => if payload != decoded_payload {return Err("Encoder error: sanity check failed.")},
            Err(e) => return Err(e),
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use GlyphLabel::*;

    #[test]
    fn encoder_symcode_from_bitvec() {
        let config = Acute32SymcodeConfig::default();
        let encoder = Acute32Encoder::new(&config);
        let mut bits = BitVec::from_bytes(&[0b01001010, 0b00000001, 0b10000011, 0b01000100]); // Will be 32 bits
        bits.truncate(30); // Only wants the first 30 bits (last two 0's are dummy)
        let symcode = encoder.encode(bits, 5).unwrap();
        assert_eq!(symcode, &[ArrowDD, TriforceR, LongDU, LongLL, ArrowRR]);
    }
}