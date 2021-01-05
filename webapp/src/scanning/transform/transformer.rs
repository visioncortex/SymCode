use visioncortex::{ColorImage, PointF64, PointI32};
use web_sys::console;

use crate::scanning::FinderCandidate;

use super::TransformFitter;

pub(crate) struct Transformer {}

impl Transformer {
    const CODE_WIDTH: usize = 335;
    const CODE_HEIGHT: usize = 195;
    /// Create an image in object space that is transformed from the most likely PerspectiveTransformation according to the finder candidates
    pub(crate) fn rectify_image(image: ColorImage, finder_candidates: Vec<FinderCandidate>, transform_error_threshold: f64) -> Option<ColorImage> {
        let image_to_object = TransformFitter::from_finder_candidates(finder_candidates, transform_error_threshold)?;

        let mut rectified_image = ColorImage::new_w_h(Self::CODE_WIDTH, Self::CODE_HEIGHT);
        for x in 0..Self::CODE_WIDTH {
            for y in 0..Self::CODE_HEIGHT {
                let position_in_image_space = image_to_object.transform_inverse(PointF64::new(x as f64, y as f64));
                // To-do: bilinear interpolation
                let interp_point = PointI32::new(position_in_image_space.x as i32, position_in_image_space.y as i32);
                //console::log_1(&format!("{:?}", interp_point).into());
                rectified_image.set_pixel(x, y, &image.get_pixel_safe(interp_point.x, interp_point.y)?);
            }
        }

        Some(rectified_image)
    }
}