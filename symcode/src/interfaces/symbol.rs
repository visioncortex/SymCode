use visioncortex::BinaryImage;

/// Definition of a symbol
pub trait Symbol {

    type Label;

	fn to_label(&self) -> Self::Label; // label is probably a Enum

	fn to_image(&self) -> BinaryImage; // to be used by SymcodeGenerator
}
