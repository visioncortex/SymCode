use visioncortex::{ColorImage, PerspectiveTransform};

/// Given a correct perspective transform, scan the image to read out a series of symbols
pub trait Reader {

	type Symbol;

    fn read(&self, frame: ColorImage, transform: PerspectiveTransform) -> Result<Vec<Self::Symbol>, &'static str>;
}