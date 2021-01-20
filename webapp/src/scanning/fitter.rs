use permutator::{Combination, Permutation};
use visioncortex::{Point2,  PointF64};

use crate::{math::{PerspectiveTransform}};

use super::{SymcodeConfig, valid_pointf64_on_image};

pub trait Fitter {
    // Input = Vec<Finders>
    // Output = (rectified) BinaryImage

    /// Given a slice of PointF64 which are the potential finder points,
    /// verify the order based on the (user-defined) spatial arrangement so that invalid arrangements are
    /// not fitted in a transform.
    ///
    /// Note that perspective distortion has to be taken into account.
    fn correct_spatial_arrangement(finder_positions_image: &[PointF64]) -> bool;

    /// Defines the metric of evaluating a transform with the potential finder points.
    /// Returns the error of the input transform, it should be the smallest when the finders are in the correct positions.
    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finder_positions_image: &[PointF64], symcode_config: &SymcodeConfig) -> f64;

    /// Check if the 4 corners in the object space will map to out-of-bound points in the image space.
    ///
    /// Those are points that cannot be sampled.
    fn transform_to_image_out_of_bound(image_width: usize, image_height: usize, image_to_object: &PerspectiveTransform, symcode_config: &SymcodeConfig) -> bool {
        let w = (symcode_config.code_width-1) as f64;
        let h = (symcode_config.code_height-1) as f64;
        let points_to_test = [
            PointF64::new(0.0, 0.0), PointF64::new(w, 0.0),
            PointF64::new(0.0, h), PointF64::new(w, h),
        ];

        for &point in points_to_test.iter() {
            let point_in_image_space = image_to_object.transform_inverse(point);
            
            if !valid_pointf64_on_image(point_in_image_space, image_width, image_height) {
                return true;
            }
        }
        
        false
    }
    
    /// Given finder candidates positions on the image and finder positions in the object space,
    /// find the "correct" perspective transform that maps the image space to the object space.
    ///
    /// symcode_config is used to evaluate the potential transforms.
    fn fit_transform<T>(image_width: usize, image_height: usize, finder_positions_image: Vec<Point2<T>>, symcode_config: &SymcodeConfig) -> Result<PerspectiveTransform, &str>
    where T: Copy + Into<f64>
    {
        let dst_pts = &symcode_config.finder_positions;
        let num_finders = dst_pts.len();

        if finder_positions_image.len() < num_finders {
            return Err("Fitter error: Not enough finder candidates in this frame.");
        }
        
        let mut best_transform = Err("No spatial arrangement for the finder candidates is correct");
        let mut min_error = std::f64::MAX;
        finder_positions_image.combination(num_finders).for_each(|mut c| {
            c.permutation().for_each(|src_pts| {
                let src_pts: Vec<PointF64> = src_pts.iter().map(|p| PointF64::new(p.x.into(), p.y.into())).collect();
                if Self::correct_spatial_arrangement(&src_pts) {
                    let transform = PerspectiveTransform::from_point_f64(&src_pts, dst_pts);
                    let error = Self::evaluate_transform(&transform, &src_pts, symcode_config);
                    if error < min_error {
                        best_transform = Ok(transform);
                        min_error = error;
                    }
                }
            });
        });
        if min_error > symcode_config.rectify_error_threshold {
           return Err("Minimum transform error is larger than rectify error threshold");
        }
        // Check if a "best" transform was found
        let best_transform = best_transform?;
        // Check if it maps a point to out of bound
        if Self::transform_to_image_out_of_bound(image_width, image_height, &best_transform, symcode_config) {
            Err("Transform to image out of bound.")
        } else {
            Ok(best_transform)
        }
    }
}