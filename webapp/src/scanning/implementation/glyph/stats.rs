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
    tolerance: f64,
}

impl ShapeStats {
    pub fn from_image(image: &BinaryImage, tolerance: f64) -> Self {
        // Upscale so that the image can be divided into 4 quadrants
        let horiz_mid = image.width;
        let vert_mid = image.height;
        let image = &Sampler::resample_image(image, image.width*2, image.height*2);
        let sampler = Sampler::new(image);

        let top_left = sampler.sample(0, 0, horiz_mid, vert_mid);
        let top_right = sampler.sample(horiz_mid, 0, sampler.image.width, vert_mid);
        let bot_left = sampler.sample(0, vert_mid, horiz_mid, sampler.image.height);
        let bot_right = sampler.sample(horiz_mid,vert_mid, sampler.image.width, sampler.image.height);
        Self {
            top_left,
            top_right,
            bot_left,
            bot_right,
            tolerance,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top_left + self.top_right + self.bot_left + self.bot_right == 0
    }

    pub const NUM_COMPARISONS: usize = 9;

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