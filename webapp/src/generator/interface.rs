use bit_vec::BitVec;
use visioncortex::BinaryImage;

pub trait SymcodeGenerator {
    type Symcode;
    fn generate(&self, symcode: Self::Symcode) -> BinaryImage;
}

trait Encoder {
    type Symcode;

	/// this encoder can only encode exactly N bits
	fn length(&self) -> usize;
	/// encode a bit string to a Symcode, panic if input length is not as defined
	fn encode(&self, bits: BitVec) -> Self::Symcode;
}