pub trait SymcodeGenerator {
    type SymcodeRepresentation;
    type SymcodeImage;
    fn generate(&self, symcode: Self::SymcodeRepresentation) -> Self::SymcodeImage;
}