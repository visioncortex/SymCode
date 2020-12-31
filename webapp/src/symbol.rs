use visioncortex::{ColorHsv, PointI32, Shape};

use crate::utils::{is_black, make_shape_square};

/// Actual Symbol or Candidate
pub(crate) struct Symbol {
    /// Average color in HSV
    pub(crate) color: ColorHsv,
    pub(crate) shape: Shape,
    /// absolute coordinates
    pub(crate) top_left: PointI32,
    pub(crate) bot_right: PointI32,
}

pub(crate) enum SymbolType {
    Finder,
    Glyph,
    Invalid,
}

impl Symbol {
    /// Compare this symbol with each template
    pub(crate) fn categorize(&self) -> SymbolType {
        if !is_black(&self.color) {
            SymbolType::Invalid
        } else if self.is_finder() {
            SymbolType::Finder
        } else if self.is_glyph() {
            SymbolType::Glyph
        } else {
            SymbolType::Invalid
        }
    }

    fn is_finder(&self) -> bool {
        //make_shape_square(&self.shape).is_circle()
        self.shape.is_circle()
    }

    fn is_glyph(&self) -> bool {
        true // All black symbols are glyphs for now
    }
}