use std::fmt::Debug;
use bit_vec::BitVec;

use crate::util::console_log_util;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq)]
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

    // For counting
    Last,
}

impl Default for GlyphLabel {
    fn default() -> Self {
        Self::Empty
    }
}

impl GlyphLabel {
    /// Number of valid variants (empty + all valid glyphs)
    pub fn num_variants() -> usize {
        Self::self_to_primitive(Self::Last).unwrap()
    }

    pub fn from_usize_representation(label: usize) -> Self {
        match FromPrimitive::from_usize(label) {
            Some(glyph_label) => glyph_label,
            None => {
                console_log_util(&format!("No corresponding label for {}.", label));
                panic!();
            },
        }
    }

    pub fn self_to_primitive(label: Self) -> Option<usize> {
        if label == GlyphLabel::Invalid {
            return None;
        }
        match ToPrimitive::to_usize(&label) {
            Some(primitive) => Some(primitive),
            None => {
                console_log_util(&format!("Cannot convert {:?} to primitive.", label));
                None
            }
        }
    }

    pub fn primitive_to_bit_vec(mut primitive: usize, length: usize) -> BitVec {
        let mut bit_vec = BitVec::from_elem(length, false);
        for i in (0..length).rev() {
            bit_vec.set(i, primitive % 2 == 1);
            primitive >>= 1;
        }
        bit_vec
    }

    pub fn self_to_bit_vec(label: Self, length: usize) -> Option<BitVec> {
        if let Some(primitive) = Self::self_to_primitive(label) {
            Some(Self::primitive_to_bit_vec(primitive, length))
        } else {
            None
        }
    }

    pub fn bit_vec_to_primitive(bit_vec: BitVec) -> usize {
        let mut primitive = 0;
        for i in bit_vec {
            primitive <<= 1;
            primitive += i as usize;
        }
        primitive
    }

    pub fn self_from_bit_vec(bit_vec: BitVec) -> Self {
        Self::from_usize_representation(
            Self::bit_vec_to_primitive(bit_vec)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_label_conversion_from() {
        let label = 0;
        assert_eq!(GlyphLabel::from_usize_representation(label), GlyphLabel::Empty);
        let label = 5;
        assert_eq!(GlyphLabel::from_usize_representation(label), GlyphLabel::LongRL);
        let label = 32;
        assert_eq!(GlyphLabel::from_usize_representation(label), GlyphLabel::TriforceR);
    }

    #[test]
    fn glyph_label_conversion_to_primitive() {
        let label = GlyphLabel::ArrowUU;
        assert_eq!(GlyphLabel::self_to_primitive(label), Some(20));
        let label = GlyphLabel::Invalid;
        assert_eq!(GlyphLabel::self_to_primitive(label), None);
        let label = GlyphLabel::Empty;
        assert_eq!(GlyphLabel::self_to_primitive(label), Some(0));
        let label = GlyphLabel::TriforceR;
        assert_eq!(GlyphLabel::self_to_primitive(label), Some(32));
    }

    #[test]
    fn glyph_label_primitive_to_bit_vec() {
        const LENGTH: usize = 6;

        let primitive = 0;
        assert!(!GlyphLabel::primitive_to_bit_vec(primitive, LENGTH).any());

        let primitive = 32;
        let bit_vec = GlyphLabel::primitive_to_bit_vec(primitive, LENGTH); // 100000
        assert_eq!(bit_vec.get(0).unwrap(), true);
        for i in 1..LENGTH {
            assert_eq!(bit_vec.get(i).unwrap(), false);
        }

        let primitive = 11;
        let bit_vec = GlyphLabel::primitive_to_bit_vec(primitive, LENGTH); // 001011
        assert_eq!(bit_vec.get(0).unwrap(), false);
        assert_eq!(bit_vec.get(1).unwrap(), false);
        assert_eq!(bit_vec.get(2).unwrap(), true);
        assert_eq!(bit_vec.get(3).unwrap(), false);
        assert_eq!(bit_vec.get(4).unwrap(), true);
        assert_eq!(bit_vec.get(5).unwrap(), true);
    }

    #[test]
    fn glyph_label_option_self_to_bit_vec() {
        const LENGTH: usize = 6;
        let label = GlyphLabel::Empty;
        assert!(!GlyphLabel::self_to_bit_vec(label, LENGTH).unwrap().any());

        let label = GlyphLabel::TriforceR;
        let bit_vec = GlyphLabel::self_to_bit_vec(label, LENGTH).unwrap(); // 100000
        assert_eq!(bit_vec.get(0).unwrap(), true);
        for i in 1..LENGTH {
            assert_eq!(bit_vec.get(i).unwrap(), false);
        }

        let label = GlyphLabel::Invalid;
        assert_eq!(GlyphLabel::self_to_bit_vec(label, LENGTH), None);

        let label = GlyphLabel::LongUU;
        let bit_vec = GlyphLabel::self_to_bit_vec(label, LENGTH).unwrap(); // 000100
        for i in 0..LENGTH {
            let check = i==3;
            assert_eq!(bit_vec.get(i).unwrap(), check);
        }
    }

    #[test]
    fn glyph_label_primitive_bit_vec_conversion() {
        const LENGTH: usize = 6;
        let primitive = 5;
        let mut bit_vec = BitVec::from_elem(LENGTH, false);
        bit_vec.set(3, true);
        bit_vec.set(LENGTH-1, true);
        assert_eq!(GlyphLabel::primitive_to_bit_vec(primitive, LENGTH), bit_vec);
        assert_eq!(primitive, GlyphLabel::bit_vec_to_primitive(bit_vec));

        let primitive = 15;
        let mut bit_vec = BitVec::from_elem(LENGTH, false);
        for i in 2..LENGTH {
            bit_vec.set(i, true);
        }
        assert_eq!(GlyphLabel::primitive_to_bit_vec(primitive, LENGTH), bit_vec);
        assert_eq!(primitive, GlyphLabel::bit_vec_to_primitive(bit_vec)); 
    }
}