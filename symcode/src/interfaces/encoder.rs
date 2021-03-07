use bit_vec::BitVec;

pub trait Encoder {

    type SymcodeRepresentation;
    
	/// encode a bit string to a Symcode, panic if input length is not as defined
	fn encode(&self, bits: BitVec, num_glyphs: usize) -> Result<Self::SymcodeRepresentation, &'static str>;
}