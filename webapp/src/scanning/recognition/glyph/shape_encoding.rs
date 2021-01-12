use std::cmp::Ordering;

use bit_vec::BitVec;
use visioncortex::{BinaryImage};

use super::ShapeStats;

pub struct ShapeEncoding {
    bits: BitVec,
}

impl Default for ShapeEncoding {
    fn default() -> Self {
        Self {
            bits: BitVec::from_elem(Self::LENGTH, false),
        }
    }
}

impl ShapeEncoding {
    /// 2 bits each for vertical, horizontal, and diagonal comparison
    const LENGTH: usize = 6;

    pub fn from_image(image: &BinaryImage, tolerance: f64) -> Self {
        let stats = ShapeStats::from_image(image);
        if stats.is_empty() {
            return Self::default();
        }

        let mut bits = BitVec::from_elem(Self::LENGTH, true); // start with all 1's
        
        match stats.vertical_comparison(tolerance) {
            Ordering::Less => bits.set(0, false),
            Ordering::Greater => bits.set(1, false),
            Ordering::Equal => {},
        }
        
        match stats.horizontal_comparison(tolerance) {
            Ordering::Less => bits.set(2, false),
            Ordering::Greater => bits.set(3, false),
            Ordering::Equal => {},
        }
        
        match stats.diagonal_comparison(tolerance) {
            Ordering::Less => bits.set(4, false),
            Ordering::Greater => bits.set(5, false),
            Ordering::Equal => {},
        }

        Self {
            bits
        }
    }

    pub fn diff(&self, other: &Self) -> usize {
        let (mut self_clone, mut other_clone) = (self.bits.clone(), other.bits.clone());
        self_clone.difference(&other.bits);
        other_clone.difference(&self.bits);
        self_clone.or(&other_clone);
        self_clone.into_iter().filter(|bit| *bit).count()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn shape_encoding_encoding_diff() {
        let a = &mut ShapeEncoding::default();
        let b = &mut ShapeEncoding::default();
        assert_eq!(a.diff(b), 0);

        a.bits.set(0, true);
        assert_eq!(a.diff(b), 1);
        b.bits.set(0, true);
        assert_eq!(a.diff(b), 0);

        a.bits.set(2, true);
        a.bits.set(5, true);
        b.bits.set(4, true);
        assert_eq!(a.diff(b), 3);

        b.bits.set(5, true);
        assert_eq!(a.diff(b), 2);
    }

    #[test]
    fn shape_encoding_from_image() {
        // Should be 10 01 01
        let encoding = &ShapeEncoding::from_image(
            &BinaryImage::from_string(
              &("-*\n".to_owned() +
                "--")
            ),
            0.0
        );
        println!("{:?}", encoding.bits);
        assert_eq!(encoding.bits.get(0), Some(true));
        assert_eq!(encoding.bits.get(1), Some(false));
        assert_eq!(encoding.bits.get(2), Some(false));
        assert_eq!(encoding.bits.get(3), Some(true));
        assert_eq!(encoding.bits.get(4), Some(false));
        assert_eq!(encoding.bits.get(5), Some(true));

        // Should be 11 11 10
        let encoding = &ShapeEncoding::from_image(
            &BinaryImage::from_string(
              &("*-\n".to_owned() +
                "-*")
            ),
            0.0
        );
        println!("{:?}", encoding.bits);
        assert_eq!(encoding.bits.get(0), Some(true));
        assert_eq!(encoding.bits.get(1), Some(true));
        assert_eq!(encoding.bits.get(2), Some(true));
        assert_eq!(encoding.bits.get(3), Some(true));
        assert_eq!(encoding.bits.get(4), Some(true));
        assert_eq!(encoding.bits.get(5), Some(false));
    }

    #[test]
    fn shape_encoding_empty_image() {
        // Should be 00 00 00
        let encoding = &ShapeEncoding::from_image(
            &BinaryImage::from_string(
              &("--\n".to_owned() +
                "--")
            ),
            0.0
        );
        println!("{:?}", encoding.bits);
        assert!(!encoding.bits.any());
    }
}