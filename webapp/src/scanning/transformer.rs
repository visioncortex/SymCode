use permutator::{Combination, Permutation};
use visioncortex::{BinaryImage, ColorImage, Point2, PointF32, PointF64, bilinear_interpolate};

use crate::{math::{PerspectiveTransform}, util::console_log_util};

use super::{SymcodeConfig, valid_pointf64_on_image};

pub trait Transformer {
    // Input = Vec<Finders>
    // Output = (rectified) BinaryImage

    /// Given a slice of PointF64 which are the potential finder points.
    ///
    /// Verify the order based on the (user-defined) spatial arrangement.
    fn correct_spatial_arrangement(finder_positions_image: &[PointF64]) -> bool;

    /// Defines the metric of evaluating a transform with the potential finder points.
    /// Returns the error of the input transform, it should be the smallest when the finders are in the correct positions.
    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finder_src_points: &[PointF64], check_points: &[PointF64]) -> f64;
    
    /// Given finder points in the object space and the configuration of the Symcode,
    /// calculates the check points which are used to evaluate reprojection error
    fn calculate_check_points(finder_positions_object: &[PointF64], symcode_config: &SymcodeConfig) -> Vec<PointF64>;
    
    /// Given finder candidates positions on the image and finder positions in the object space,
    /// find the "correct" perspective transform that maps the image space to the object space.
    ///
    /// symcode_config is used to evaluate the potential transforms.
    fn fit_transform<T>(finder_positions_image: Vec<Point2<T>>, symcode_config: &SymcodeConfig) -> Option<PerspectiveTransform>
    where T: Copy + Into<f64>
    {
        let dst_pts = &symcode_config.finder_positions;
        let num_finders = dst_pts.len();

        if finder_positions_image.len() < num_finders {
            console_log_util(&"Fitter error: Not enough finder candidates in this frame.");
            return None;
        }
        
        let check_pts: Vec<PointF64> = Self::calculate_check_points(dst_pts, symcode_config);
        
        let mut best_transform = PerspectiveTransform::default();
        let mut min_error = std::f64::MAX;
        finder_positions_image.combination(num_finders).for_each(|mut c| {
            c.permutation().for_each(|src_pts| {
                let src_pts: Vec<PointF64> = src_pts.iter().map(|p| PointF64::new(p.x.into(), p.y.into())).collect();
                if Self::correct_spatial_arrangement(&src_pts) {
                    let transform = PerspectiveTransform::from_point_f64(&src_pts, dst_pts);
                    let error = Self::evaluate_transform(&transform, &src_pts, &check_pts);
                    if error < min_error {
                        best_transform = transform;
                        min_error = error;
                    }
                }
            });
        });
        if min_error > symcode_config.rectify_error_threshold {
            None
        } else {
            Some(best_transform)
        }
    }
    /// Check if the 4 corners in the object space will map to out-of-bound points in the image space.
    ///
    /// Those are points that cannot be sampled.
    fn transform_to_image_out_of_bound(image: &ColorImage, image_to_object: &PerspectiveTransform, symcode_config: &SymcodeConfig) -> bool {
        let w = (symcode_config.code_width-1) as f64;
        let h = (symcode_config.code_height-1) as f64;
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

    /// This function will be used to binarize the rectified input image
    fn binarize_image(image: &ColorImage) -> BinaryImage;

    /// Rectify the input image into object space and binarize it
    fn transform_image<T: std::marker::Copy + Into<f64>>(image: ColorImage, finder_positions_image: Vec<Point2<T>>, symcode_config: &SymcodeConfig) -> Option<BinaryImage> {
        let image_to_object = Self::fit_transform(finder_positions_image, symcode_config)?;
        if Self::transform_to_image_out_of_bound(&image, &image_to_object, symcode_config) {
            return None;
        }

        let code_width = symcode_config.code_width;
        let code_height = symcode_config.code_height;

        let mut rectified_image = ColorImage::new_w_h(code_width, code_height);
        // For each point in object space
        for x in 0..code_width {
            for y in 0..code_height {
                // Obtains the sample point in image space
                let position_in_image_space = image_to_object.transform_inverse(PointF64::new(x as f64, y as f64));
                let position_in_image_space = PointF32::new(position_in_image_space.x as f32, position_in_image_space.y as f32);

                // Interpolate the color there
                rectified_image.set_pixel(x, y,
                    &bilinear_interpolate(&image, position_in_image_space)
                );
            }
        }

        Some(Self::binarize_image(&rectified_image))
    }
}