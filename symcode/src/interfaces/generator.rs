use visioncortex::BinaryImage;

pub trait SymcodeGenerator {

    type SymcodeRepresentation;

    fn generate(&self, symcode: Self::SymcodeRepresentation) -> BinaryImage;
}