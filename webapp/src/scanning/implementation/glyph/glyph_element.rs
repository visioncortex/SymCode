use std::fmt::Debug;
use visioncortex::BinaryImage;

use crate::util::console_log_util;

use super::{GlyphTrace};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
/// Useful for testing purposes only.
///
/// For a given alphabet image, the index should go from top to bottom, left to right.
pub enum GlyphLabel {
    Invalid = -1,
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

    DoubleTallDiamond,
    StackedFatDiamond,
    FourTriangles,
    FourKites,

    ArrowRR,
    ArrowDD,
    ArrowLL,
    ArrowUU,

    ArrowRL,
    ArrowDU,
    FatDiamond,
    TallDiamond,

    SmallTripleU,
    SmallTripleR,
    SmallTripleD,
    SmallTripleL,

    TriforceD,
    TriforceL,
    TriforceU,
    TriforceR,
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
                console_log_util(&format!("No corresponding label for {}.", label));
                panic!();
            },
        }
    }
}

pub struct Glyph {
    pub image: BinaryImage,
    pub label: GlyphLabel,
    pub encoding: GlyphTrace,
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
        let encoding = GlyphTrace::from_image(&image, stat_tolerance);
        Self {
            image,
            label,
            encoding,
        }
    }
}