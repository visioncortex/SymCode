use visioncortex::{BoundingRect, PerspectiveTransform};

/// Given an array of finder candidates positions, evaluate the "correct" perspective transform that
/// maps the image space to the object space.
pub trait Fitter {
    fn fit(
    	&self, finder_positions: Vec<BoundingRect>, image_width: usize, image_height: usize
    ) -> Result<PerspectiveTransform, &str>;
}