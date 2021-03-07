use visioncortex::{BinaryImage, BoundingRect, ColorImage, Shape};

pub trait Finder {
    fn find(&self, input: &ColorImage) -> Result<Vec<BoundingRect>, &'static str>;
}

pub trait FinderElement {

	fn to_image(&self, width: usize, height: usize) -> BinaryImage; // to be used by SymcodeGenerator

	fn is_finder(&self, shape: Shape) -> bool; // to be used by SymcodeScanner
}
