use bit_vec::BitVec;
use visioncortex::{BinaryImage, ColorImage, Shape};

pub trait SymcodeScanner {
    type SymcodeRepresentation;
    type Err;

    fn scan_and_decode(&self, image: ColorImage) -> Result<BitVec, Self::Err> {
        self.decode(self.scan(image)?)
    }
    fn scan(&self, image: ColorImage) -> Result<Self::SymcodeRepresentation, Self::Err>;
    fn decode(&self, symcode: Self::SymcodeRepresentation) -> Result<BitVec, Self::Err>;
}

pub trait Finder {
	fn to_image(&self, width: usize, height: usize) -> BinaryImage; // to be used by SymcodeGenerator
	fn is_finder(&self, shape: Shape) -> bool; // to be used by SymcodeScanner
}