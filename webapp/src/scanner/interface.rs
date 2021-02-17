use bit_vec::BitVec;
use visioncortex::{BinaryImage, ColorImage, Shape};

pub trait SymcodeScanner {
    type Symcode;
    type Err;

    fn scan_and_decode(&self, image: ColorImage) -> Result<BitVec, Self::Err> {
        self.decode(self.scan(image)?)
    }
    fn scan(&self, image: ColorImage) -> Result<Self::Symcode, Self::Err>;
    fn decode(&self, symcode: Self::Symcode) -> Result<BitVec, Self::Err>;
}

pub trait Finder {
	fn to_image(&self) -> BinaryImage; // to be used by SymcodeGenerator
	fn is_finder(&self, shape: Shape) -> bool; // to be used by SymcodeScanner
}