use visioncortex::{ColorImage, PointF32, PointF64, bilinear_interpolate};
use web_sys::console;

use crate::{math::PerspectiveTransform, scanning::{FinderCandidate, GlyphCode, valid_pointf64_on_image}};

use super::TransformFitter;

pub(crate) struct Transformer {}

impl Transformer {
    /// Create an image in object space that is transformed from the most likely PerspectiveTransformation according to the finder candidates
    pub(crate) fn rectify_image(image: ColorImage, finder_candidates: Vec<FinderCandidate>, transform_error_threshold: f64) -> Option<ColorImage> {
        let image_to_object = TransformFitter::from_finder_candidates(finder_candidates, transform_error_threshold)?;

        if Self::transform_to_image_out_of_bound(&image, &image_to_object) {
            return None;
        }


        let mut rectified_image = ColorImage::new_w_h(GlyphCode::WIDTH, GlyphCode::HEIGHT);
        for x in 0..GlyphCode::WIDTH {
            for y in 0..GlyphCode::HEIGHT {
                let position_in_image_space = image_to_object.transform_inverse(PointF64::new(x as f64, y as f64));
                let position_in_image_space = PointF32::new(position_in_image_space.x as f32, position_in_image_space.y as f32);

                // console::log_1(&format!("{} {} {} {}", 
                //     position_in_image_space.x.floor() as usize,
                //     position_in_image_space.x.ceil() as usize,
                //     position_in_image_space.y.floor() as usize,
                //     position_in_image_space.y.ceil() as usize,
                //     ).into());
                // console::log_1(&format!("{} {}", image.width, image.height).into());

                rectified_image.set_pixel(x, y,
                    &bilinear_interpolate(&image, position_in_image_space)
                );
            }
        }

        Some(rectified_image)
    }

    /// Check if the 4 corners in the object space will map to a out-of-bound point in the image space
    fn transform_to_image_out_of_bound(image: &ColorImage, image_to_object: &PerspectiveTransform) -> bool {
        let w = (GlyphCode::WIDTH-1) as f64;
        let h = (GlyphCode::HEIGHT-1) as f64;
        let points_to_test = [
            PointF64::new(0.0, 0.0), PointF64::new(0.0, h),
            PointF64::new(w, 0.0), PointF64::new(w, h),
        ];

        for &point in points_to_test.iter() {
            let point_in_image_space = image_to_object.transform_inverse(point);
            
            //console::log_1(&format!("{:?}", point_in_image_space).into());
            if !valid_pointf64_on_image(point_in_image_space, image) {
                return true;
            }
        }
        
        false
    }
}