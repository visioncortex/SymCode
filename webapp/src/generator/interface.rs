use bit_vec::BitVec;
use visioncortex::BinaryImage;

pub trait SymcodeGenerator {
    type SymcodeRepresentation;
    fn generate(&self, symcode: Self::SymcodeRepresentation) -> BinaryImage;
}

trait Encoder {
    type SymcodeRepresentation;

	/// this encoder can only encode exactly N bits
	fn length(&self) -> usize;
	/// encode a bit string to a Symcode, panic if input length is not as defined
	fn encode(&self, bits: BitVec) -> Self::SymcodeRepresentation;
}