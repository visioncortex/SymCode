use visioncortex::{BinaryImage, Shape};

pub trait Finder {
	fn to_image(&self, width: usize, height: usize) -> BinaryImage; // to be used by SymcodeGenerator
	fn is_finder(&self, shape: Shape) -> bool; // to be used by SymcodeScanner
}