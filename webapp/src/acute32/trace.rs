use bit_vec::BitVec;
use visioncortex::BinaryImage;
use stats::ShapeStats;
use std::cmp::Ordering;

pub trait Trace {
    fn bits(&self) -> &BitVec;

    /// the default implementation is to XOR the two bit strings and count the number of 1s
    fn diff(&self, other: &Self) -> usize {
        let (mut self_clone, mut other_clone) = (self.bits().clone(), other.bits().clone());
        self_clone.difference(&other.bits());
        other_clone.difference(&self.bits());
        self_clone.or(&other_clone);
        self_clone.into_iter().filter(|bit| *bit).count()
    }


    fn from_image(image: &BinaryImage, tolerance: f64) -> Self;
}

#[derive(Debug)]
pub struct GlyphTrace {
    pub bits: BitVec,
}

impl Trace for GlyphTrace {
    fn bits(&self) -> &BitVec {
        &self.bits
    }

    fn from_image(image: &BinaryImage, tolerance: f64) -> Self {
        let mut layer_traces = vec![];
        // Encode each small layer
        // layer_traces = Self::subdivide_and_encode(image, tolerance);
        // Encode the big layer
        layer_traces.push(LayerTrace::from_image(image, tolerance));
        Self::from_layer_traces(layer_traces)
    }
}

impl GlyphTrace {
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

    #[allow(unused_assignments)]
    fn from_image(image: &BinaryImage, tolerance: f64) -> Self {
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
        set_bits!(ef_gh_comparison);

        Self {
            bits
        }
    }
}

impl Default for LayerTrace {
    fn default() -> Self {
        Self {
            bits: BitVec::from_elem(Self::LENGTH, false),
        }
    }
}

// Partition the symbol image into 2x2 = 4 big blocks
// Denote top-left, top-right, bottom-left, bottom-right weights by a,b,c,d respectively
// Partition the symbol image into 4x4 = 16 small blocks
// Denote top two, bottom two, left two, right two weights by e,f,g,h respectively
impl LayerTrace {
    /// 2 bits each for a+b <> c+d, a+c <> b+d, a+d <> b+c, a <> b, c <> d, a <> c, b <> d, a <> d, b <> c, e+f <> g+h
    const LENGTH: usize = ShapeStats::NUM_COMPARISONS << 1;
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

mod stats {
    use std::cmp::Ordering;

    use visioncortex::{BinaryImage, Sampler};

    #[derive(Debug)]
    pub struct ShapeStats {
        // a
        top_left: usize,
        // b
        top_right: usize,
        // c
        bot_left: usize,
        // d
        bot_right: usize,
        // e
        top: usize,
        // f
        bottom: usize,
        // g
        left: usize,
        // h
        right: usize,
        tolerance: f64,
    }

    impl ShapeStats {
        pub fn from_image(image: &BinaryImage, tolerance: f64) -> Self {
            // Upscale so that the image can be divided into 4x4 = 16 blocks
            let horiz_q1 = image.width;
            let vert_q1 = image.height;
            let horiz_mid = image.width << 1;
            let vert_mid = image.height << 1;
            let horiz_q3 = horiz_mid + horiz_q1;
            let vert_q3 = vert_mid + vert_q1;

            let image = &Sampler::resample_image(image, image.width*4, image.height*4);
            let sampler = Sampler::new(image);

            let top_left = sampler.sample(0, 0, horiz_mid, vert_mid);
            let top_right = sampler.sample(horiz_mid, 0, sampler.image.width, vert_mid);
            let bot_left = sampler.sample(0, vert_mid, horiz_mid, sampler.image.height);
            let bot_right = sampler.sample(horiz_mid,vert_mid, sampler.image.width, sampler.image.height);
           
            let top = sampler.sample(horiz_q1, 0, horiz_q3, vert_q1);
            let bottom = sampler.sample(horiz_q1, vert_q3, horiz_q3, sampler.image.height);
            let left = sampler.sample(0, vert_q1, horiz_q1, vert_q3);
            let right = sampler.sample(horiz_q3, vert_q1, sampler.image.width, vert_q3);

            Self {
                top_left,
                top_right,
                bot_left,
                bot_right,
                top,
                bottom,
                left,
                right,
                tolerance,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.top_left + self.top_right + self.bot_left + self.bot_right == 0
        }

        pub const NUM_COMPARISONS: usize = 10;

        /// Returns an Ordering based on top vs bottom
        pub fn vertical_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top_left + self.top_right, self.bot_left + self.bot_right, self.tolerance)
        }

        /// Returns an Ordering based on left vs right
        pub fn horizontal_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top_left + self.bot_left, self.top_right + self.bot_right, self.tolerance)
        }

        /// Returns an Ordering based on backslash vs slash
        pub fn diagonal_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top_left + self.bot_right, self.top_right + self.bot_left, self.tolerance)
        }

        pub fn a_b_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top_left, self.top_right, self.tolerance)
        }

        pub fn c_d_comparison(&self) -> Ordering {
            Self::approximate_compare(self.bot_left, self.bot_right, self.tolerance)
        }

        pub fn a_c_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top_left, self.bot_left, self.tolerance)
        }

        pub fn b_d_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top_right, self.bot_right, self.tolerance)
        }

        pub fn a_d_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top_left, self.bot_right, self.tolerance)
        }

        pub fn b_c_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top_right, self.bot_left, self.tolerance)
        }

        pub fn ef_gh_comparison(&self) -> Ordering {
            Self::approximate_compare(self.top + self.bottom, self.left + self.right, self.tolerance)
        }

        /// The higher the tolerance, the easier it is for a,b to be considered equal
        fn approximate_compare(a: usize, b: usize, tolerance: f64) -> Ordering {
            if a == b {
                return Ordering::Equal;
            }
            let determinant = std::cmp::min(a,b) as f64 / std::cmp::max(a,b) as f64;
            if determinant > (1.0-tolerance) {
                return Ordering::Equal;
            }
            if a > b {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
}