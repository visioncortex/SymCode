use std::cmp::Ordering;

use bit_vec::BitVec;
use visioncortex::{BinaryImage, PointI32};

use super::ShapeStats;

pub trait Trace {
    fn bits(&self) -> &BitVec;
    fn diff(&self, other: &Self) -> usize {
        let (mut self_clone, mut other_clone) = (self.bits().clone(), other.bits().clone());
        self_clone.difference(&other.bits());
        other_clone.difference(&self.bits());
        self_clone.or(&other_clone);
        self_clone.into_iter().filter(|bit| *bit).count()
    }
}

#[derive(Debug)]
pub struct GlyphTrace {
    pub bits: BitVec,
}

impl Trace for GlyphTrace {
    fn bits(&self) -> &BitVec {
        &self.bits
    }
}

impl GlyphTrace {

    pub fn from_image(image: &BinaryImage, tolerance: f64) -> Self {
        let mut layer_traces = vec![];
        // Encode each small layer
        // layer_traces = Self::subdivide_and_encode(image, tolerance);
        // Encode the big layer
        layer_traces.push(LayerTrace::from_image(image, tolerance));
        Self::from_layer_traces(layer_traces)
    }

    /// Returns a vector of 4 LayerTrace in order of top-left, top-right, bot-right, bot-left
    fn subdivide_and_encode(image: &BinaryImage, tolerance: f64) -> Vec<LayerTrace> {
        let horiz_mid = image.width as i32;
        let vert_mid = image.height as i32;
        let image = &visioncortex::Sampler::resample_image(image, image.width*2, image.height*2);
        [PointI32::new(0, 0), PointI32::new(horiz_mid, 0), PointI32::new(horiz_mid, vert_mid), PointI32::new(0, vert_mid)].iter()
            .map(|top_left| {
                let rect = visioncortex::BoundingRect::new_x_y_w_h(top_left.x, top_left.y, horiz_mid, vert_mid);
                let crop = image.crop_with_rect(rect);
                LayerTrace::from_image(&crop, tolerance)
            })
            .collect()
    }

    pub fn from_layer_traces(layer_traces: Vec<LayerTrace>) -> Self {
        let total_length = layer_traces.len() * LayerTrace::LENGTH;
        Self {
            bits: BitVec::from_fn(total_length, |i| {layer_traces[i/LayerTrace::LENGTH].bits.get(i%LayerTrace::LENGTH).unwrap()}),
        }
    }
}

#[derive(Debug)]
pub struct LayerTrace {
    pub bits: BitVec,
}

impl Trace for LayerTrace {
    fn bits(&self) -> &BitVec {
        &self.bits
    }
}

impl Default for LayerTrace {
    fn default() -> Self {
        Self {
            bits: BitVec::from_elem(Self::LENGTH, false),
        }
    }
}

// Denote top-left, top-right, bottom-left, bottom-right weights by a,b,c,d respectively
impl LayerTrace {
    /// 2 bits each for a+b <> c+d, a+c <> b+d, a+d <> b+c, a <> b, c <> d, a <> c, b <> d, a <> d, b <> c
    const LENGTH: usize = ShapeStats::NUM_COMPARISONS << 1;

    #[allow(unused_assignments)]
    pub fn from_image(image: &BinaryImage, tolerance: f64) -> Self {
        let stats = ShapeStats::from_image(image, tolerance);
        if stats.is_empty() {
            return Self::default();
        }

        let mut bits = BitVec::from_elem(Self::LENGTH, true); // start with all 1's

        let mut bit_offset = 0;
        macro_rules! set_bits {
            ($fun:ident) => {
                match stats.$fun() {
                    Ordering::Less => bits.set(bit_offset, false),
                    Ordering::Greater => bits.set(bit_offset+1, false),
                    Ordering::Equal => {},
                }
                bit_offset += 2;
            }
        }

        set_bits!(vertical_comparison);
        set_bits!(horizontal_comparison);
        set_bits!(diagonal_comparison);
        set_bits!(a_b_comparison);
        set_bits!(c_d_comparison);
        set_bits!(a_c_comparison);
        set_bits!(b_d_comparison);
        set_bits!(a_d_comparison);
        set_bits!(b_c_comparison);

        Self {
            bits
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn shape_encoding_encoding_diff() {
        let a = &mut LayerTrace::default();
        let b = &mut LayerTrace::default();
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
        let encoding = &LayerTrace::from_image(
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
        let encoding = &LayerTrace::from_image(
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
        let encoding = &LayerTrace::from_image(
            &BinaryImage::from_string(
              &("--\n".to_owned() +
                "--")
            ),
            0.0
        );
        println!("{:?}", encoding.bits);
        assert!(!encoding.bits.any());
    }

    #[test]
    fn glyph_encoding_empty() {
        let encoding = &GlyphTrace::from_image(
            &BinaryImage::from_string(
              &("----\n".to_owned() +
                "----\n" +
                "----\n" +
                "----\n"
              )
            ),
            0.0
        );
        assert!(!encoding.bits.any());
    }

    #[test]
    fn glyph_encoding_typical() {
        let encoding = &GlyphTrace::from_image(
            &BinaryImage::from_string(
              &("*---\n".to_owned() +
                "--*-\n" +
                "*--*\n" +
                "--*-\n"
              )
            ),
            0.0
        );
        let i = 0;
        assert_eq!(encoding.bits.get(i), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+1), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+2), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+3), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+4), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+5), Some(false)); // 0

        let i = 6;
        assert_eq!(encoding.bits.get(i), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+1), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+2), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+3), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+4), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+5), Some(true)); // 1

        let i = 12;
        assert_eq!(encoding.bits.get(i), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+1), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+2), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+3), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+4), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+5), Some(true)); // 1

        let i = 18;
        assert_eq!(encoding.bits.get(i), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+1), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+2), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+3), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+4), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+5), Some(false)); // 0

        let i = 24;
        assert_eq!(encoding.bits.get(i), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+1), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+2), Some(false)); // 0
        assert_eq!(encoding.bits.get(i+3), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+4), Some(true)); // 1
        assert_eq!(encoding.bits.get(i+5), Some(false)); // 0
    }
}