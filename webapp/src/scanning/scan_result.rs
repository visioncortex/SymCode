use crate::symbol::Symbol;

pub struct ScanResult {
    /// What look like the Finder patterns
    pub(crate) finders: Vec<Symbol>,
    /// What look like the data-carrying Glyphs
    pub(crate) glyphs: Vec<Symbol>,
}