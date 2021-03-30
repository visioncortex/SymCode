use bit_vec::BitVec;
use visioncortex::ColorImage;

/// The scanning pipeline
pub trait SymcodeScanner {

    type SymcodeRepresentation;

    type Err;

    fn scan_and_decode(&self, image: ColorImage) -> Result<BitVec, Self::Err> {
        self.decode(self.scan(image)?)
    }

    fn scan(&self, image: ColorImage) -> Result<Self::SymcodeRepresentation, Self::Err>;

    fn decode(&self, symcode: Self::SymcodeRepresentation) -> Result<BitVec, Self::Err>;
}
