use visioncortex::{BoundingRect, PerspectiveTransform};

pub trait Fitter {
    fn fit(
    	&self, finder_positions: Vec<BoundingRect>, image_width: usize, image_height: usize
    ) -> Result<PerspectiveTransform, &str>;
}