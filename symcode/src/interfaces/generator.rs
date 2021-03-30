use visioncortex::BinaryImage;

/// To generate a Symcode image for a given Symcode representation
pub trait SymcodeGenerator {

    type SymcodeRepresentation;

    fn generate(&self, symcode: Self::SymcodeRepresentation) -> BinaryImage;
}