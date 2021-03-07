use visioncortex::{ColorImage, PerspectiveTransform};

pub trait Reader {

	type Symbol;

    fn read(&self, frame: ColorImage, transform: PerspectiveTransform) -> Result<Vec<Self::Symbol>, &'static str>;
}