use crate::scanning::{GlyphReader};

use super::{GlyphLabel, GlyphLibrary};

/// An implementation of GlyphReader
pub struct Recognizer;

impl GlyphReader for Recognizer {
    type Label = GlyphLabel;
}