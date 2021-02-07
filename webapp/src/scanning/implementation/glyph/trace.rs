use std::cmp::Ordering;

use bit_vec::BitVec;
use visioncortex::{BinaryImage, PointI32};

use crate::scanning::Trace;
use super::ShapeStats;

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
        set_bits!(a_c_comparison);
        set_bits!(a_d_comparison);
        set_bits!(b_c_comparison);
        set_bits!(b_d_comparison);
        set_bits!(c_d_comparison);

        Self {
            bits
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const T: bool = true;
    const F: bool = false;

    #[test]
    fn trace_diff() {
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
    fn layer_trace_from_image() {
        // Should be 10 01 01 01 11 11 10 10 11
        let encoding = &LayerTrace::from_image(
            &BinaryImage::from_string(
              &("-*\n".to_owned() +
                "--")
            ),
            0.0
        );
        assert!(encoding.bits.eq_vec(&[
            T,F,F,T,F,T,F,T,T,T,T,T,T,F,T,F,T,T
        ]));

        // Should be 11 11 10 10 10 11 11 01 01
        let encoding = &LayerTrace::from_image(
            &BinaryImage::from_string(
              &("*-\n".to_owned() +
                "-*")
            ),
            0.0
        );
        assert!(encoding.bits.eq_vec(&[
            T,T,T,T,T,F,T,F,T,F,T,T,T,T,F,T,F,T
        ]));
    }

    #[test]
    fn layer_trace_empty_image() {
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
    fn glyph_trace_empty() {
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
    fn glyph_trace_typical() {
        // Should be 01 01 10 11 11 01 11 01 01
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

        assert!(encoding.bits.eq_vec(&[
            F,T,F,T,T,F,T,T,T,T,F,T,T,T,F,T,F,T
        ]));
    }
}