use std::cmp::Ordering;

use visioncortex::{BinaryImage, Sampler};

#[derive(Debug)]
pub struct ShapeStats {
    top_left: usize,
    top_right: usize,
    bot_right: usize,
    bot_left: usize,
}

impl ShapeStats {
    pub fn from_image(image: &BinaryImage) -> Self {
        // Upscale so that the image can be divided into 4 quadrants
        let image = &Sampler::resample_image(image, image.width*2, image.height*2);
        let sampler = Sampler::new(image);
        let horiz_mid = image.width/2;
        let vert_mid = image.height/2;

        let top_left = sampler.sample(0, 0, horiz_mid, vert_mid);
        let top_right = sampler.sample(horiz_mid, 0, sampler.image.width, vert_mid);
        let bot_left = sampler.sample(0, vert_mid, horiz_mid, sampler.image.height);
        let bot_right = sampler.sample(horiz_mid,vert_mid, sampler.image.width, sampler.image.height);
        Self {
            top_left,
            top_right,
            bot_left,
            bot_right,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top_left + self.top_right + self.bot_left + self.bot_right == 0
    }

    /// Returns an Ordering based on top vs bottom
    pub fn vertical_comparison(&self, tolerance: f64) -> Ordering {
        Self::approximate_compare(self.top_left + self.top_right, self.bot_left + self.bot_right, tolerance)
    }

    /// Returns an Ordering based on left vs right
    pub fn horizontal_comparison(&self, tolerance: f64) -> Ordering {
        Self::approximate_compare(self.top_left + self.bot_left, self.top_right + self.bot_right, tolerance)
    }

    /// Returns an Ordering based on backslash vs slash
    pub fn diagonal_comparison(&self, tolerance: f64) -> Ordering {
        Self::approximate_compare(self.top_left + self.bot_right, self.top_right + self.bot_left, tolerance)
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