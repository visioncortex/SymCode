use std::fmt::Debug;

use visioncortex::BinaryImage;

use crate::interfaces::symbol::Symbol as SymbolInterface;

use super::{GlyphLabel, GlyphTrace};

pub struct Symbol {
    pub image: BinaryImage,
    pub label: GlyphLabel,
    pub encoding: GlyphTrace, 
}

impl SymbolInterface for Symbol {
    type Label = GlyphLabel;

    fn to_label(&self) -> Self::Label {
        self.label
    }

    fn to_image(&self) -> visioncortex::BinaryImage {
        self.image.clone()
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Glyph")
            .field("label", &self.label)
            .field("encoding", &self.encoding)
            .finish()
    }
}

impl Symbol {
    pub fn from_image_label(image: BinaryImage, label: GlyphLabel, stat_tolerance: f64) -> Self {
        let encoding = GlyphTrace::from_image(&image, stat_tolerance);
        Self {
            image,
            label,
            encoding,
        }
    }
}