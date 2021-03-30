use bit_vec::BitVec;

/// To encode a bit string into a Symcode Representation
pub trait Encoder {

    type SymcodeRepresentation;
    
	/// encode `bits` into `Symcode`, panic if input length is not as defined
	fn encode(&self, bits: BitVec, num_glyphs: usize) -> Result<Self::SymcodeRepresentation, &'static str>;
}