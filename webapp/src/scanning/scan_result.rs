use std::iter::Scan;

use super::{Symbol, SymbolCategory};

pub struct ScanResult {
    /// What look like the Finder patterns
    pub(crate) finders: Vec<Symbol>,
    /// What look like the data-carrying Glyphs
    pub(crate) glyphs: Vec<Symbol>,
}

impl ScanResult {
    pub(crate) fn from_vec_of_symbol(symbols: Vec<Symbol>) -> Self {
        let mut result = ScanResult { finders: vec![], glyphs: vec![] };
        for symbol in symbols.into_iter() {result.insert_symbol(symbol)}
        result
    }
    
    fn insert_symbol(&mut self, symbol: Symbol) {
        match symbol.category {
            SymbolCategory::Finder => self.finders.push(symbol),
            SymbolCategory::Glyph => self.glyphs.push(symbol),
            SymbolCategory::Invalid => {},
        }
    }
}