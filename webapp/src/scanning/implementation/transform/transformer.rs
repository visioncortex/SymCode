use visioncortex::{ColorImage, PointF32, PointF64, PointI32, bilinear_interpolate};

use crate::{math::PerspectiveTransform, scanning::{FinderCandidate, GlyphCode, valid_pointf64_on_image}, util::console_log_util};

use super::TransformFitter;

pub(crate) struct Transformer {}

impl Transformer {
    /// Create an image in object space that is transformed from the most likely PerspectiveTransformation according to the finder candidates
    pub(crate) fn rectify_image(image: ColorImage, finder_candidates: Vec<PointI32>, transform_error_threshold: f64) -> Option<ColorImage> {
        let image_to_object = TransformFitter::from_finder_candidates(finder_candidates, transform_error_threshold)?;
        
        if Self::transform_to_image_out_of_bound(&image, &image_to_object) {
            console_log_util("transform out of bounds");
            return None;
        }

        let mut rectified_image = ColorImage::new_w_h(GlyphCode::CODE_WIDTH, GlyphCode::CODE_HEIGHT);
        // For each point in object space
        for x in 0..GlyphCode::CODE_WIDTH {
            for y in 0..GlyphCode::CODE_HEIGHT {
                // Obtains the sample point in image space
                let position_in_image_space = 
                    image_to_object.transform_inverse(PointF64::new(x as f64, y as f64)).to_point_f32();

                // Interpolate the color there
                rectified_image.set_pixel(x, y, &image.sample_pixel_at(position_in_image_space));
            }
        }

        Some(rectified_image)
    }

    /// Check if the 4 corners in the object space will map to out-of-bound points in the image space.
    ///
    /// Those are points that cannot be sampled.
    fn transform_to_image_out_of_bound(image: &ColorImage, image_to_object: &PerspectiveTransform) -> bool {
        let w = (GlyphCode::CODE_WIDTH-1) as f64;
        let h = (GlyphCode::CODE_HEIGHT-1) as f64;
        let points_to_test = [
            PointF64::new(0.0, 0.0), PointF64::new(w, 0.0),
            PointF64::new(0.0, h), PointF64::new(w, h),
        ];

        for &point in points_to_test.iter() {
            let point_in_image_space = image_to_object.transform_inverse(point);
            
            if !valid_pointf64_on_image(point_in_image_space, image) {
                return true;
            }
        }
        
        false
    }
}