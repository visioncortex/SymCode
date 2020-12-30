use bit_vec::BitVec;
use visioncortex::ColorImage;

pub trait Parser {
    fn parse(im: ColorImage) -> BitVec;
}