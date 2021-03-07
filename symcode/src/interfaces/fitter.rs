use visioncortex::{BoundingRect, PerspectiveTransform};

pub trait Fitter {
    fn process(
    	&self, finder_positions_image: Vec<BoundingRect>, raw_image_width: usize, raw_image_height: usize
    ) -> Result<PerspectiveTransform, &str>;
}