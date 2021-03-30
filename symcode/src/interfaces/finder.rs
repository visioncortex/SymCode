use visioncortex::{BinaryImage, BoundingRect, ColorImage, Shape};

/// To detect finder elements from a color image
pub trait Finder {
	/// If succeed, return an array of finder positions
    fn find(&self, input: &ColorImage) -> Result<Vec<BoundingRect>, &'static str>;
}

/// Definition of a finder element
pub trait FinderElement {

	fn to_image(&self, width: usize, height: usize) -> BinaryImage; // to be used by SymcodeGenerator

	fn is_finder(&self, shape: Shape) -> bool; // to be used by SymcodeScanner
}
