use std::fmt::Debug;
use visioncortex::BinaryImage;
use web_sys::console;

use super::{ShapeEncoding};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, FromPrimitive)]
/// Useful for testing purposes only.
///
/// For a given alphabet image, the index should go from top to bottom, left to right.
pub enum GlyphLabel {
    Empty = 0,

    LongRR,
    LongDD,
    LongLL,
    LongUU,

    LongRL,
    LongDU,
    LongLR,
    LongUD,

    SmallDoubleUD,
    SmallDoubleRL,
    SmallDoubleDU,
    SmallDoubleLR,

    SmallTripleU,
    SmallTripleR,
    SmallTripleD,
    SmallTripleL,
}

impl Default for GlyphLabel {
    fn default() -> Self {
        Self::Empty
    }
}

impl GlyphLabel {
    /// Will be replaced by sth like FromPrimitive
    pub fn from_usize_representation(label: usize) -> Self {
        match FromPrimitive::from_usize(label) {
            Some(glyph_label) => glyph_label,
            None => {
                console::log_1(&format!("No corresponding label for {}.", label).into());
                panic!();
            },
        }
    }
}

pub struct Glyph {
    pub image: BinaryImage,
    pub label: GlyphLabel,
    pub encoding: ShapeEncoding,
}

impl Debug for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Glyph")
            .field("label", &self.label)
            .field("encoding", &self.encoding)
            .finish()
    }
}

impl Glyph {
    pub fn from_image_label(image: BinaryImage, label: GlyphLabel, stat_tolerance: f64) -> Self {
        let encoding = ShapeEncoding::from_image(&image, stat_tolerance);
        Self {
            image,
            label,
            encoding,
        }
    }
}