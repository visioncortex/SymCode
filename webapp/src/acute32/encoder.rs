use bit_vec::BitVec;

use crate::interfaces::encoder::Encoder as EncoderInterface;

use super::GlyphLabel;

#[derive(Default)]
pub struct Acute32Encoder;

impl EncoderInterface for Acute32Encoder {
    type SymcodeRepresentation = Vec<GlyphLabel>;

    fn encode(&self, bits: bit_vec::BitVec, num_symbols: usize) -> Self::SymcodeRepresentation {
        let symbol_num_bits = crate::math::num_bits(GlyphLabel::num_variants());
        if bits.len() != symbol_num_bits*num_symbols {
            panic!("Input bits length and self-defined length do not agree!");
        }

        let mut result: Self::SymcodeRepresentation = Vec::with_capacity(num_symbols);

        for i in 0..num_symbols {
            let symbol_bit_vec = BitVec::from_fn(symbol_num_bits, |j| { // j is in [0, symbol_num_bits-1]
                let index = i*symbol_num_bits + j;
                bits[index]
            });
            result.push(GlyphLabel::self_from_bit_vec(symbol_bit_vec));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use GlyphLabel::*;

    #[test]
    fn encoder_symcode_from_bitvec() {
        let encoder = Acute32Encoder {}; 
        let mut bits = BitVec::from_bytes(&[0b01001010, 0b00000001, 0b10000011, 0b01000100]); // Will be 32 bits
        bits.truncate(30); // Only wants the first 30 bits (last two 0's are dummy)
        let symcode = encoder.encode(bits, 5);
        assert_eq!(symcode, &[ArrowDD, TriforceR, LongDU, LongLL, ArrowRR]);
    }
}