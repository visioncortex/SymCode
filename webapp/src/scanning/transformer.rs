use permutator::{Combination, Permutation};
use visioncortex::{Point2, PointF64};

use crate::{math::{PerspectiveTransform, euclid_dist_f64, normalize_point_f64}, util::console_log_util};

use super::SymcodeConfig;

pub trait Transformer {
    // Input = Vec<Finders>
    // Output = Image

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
    fn fit_transform<T,U>(finder_positions_image: Vec<Point2<T>>, finder_positions_object: Vec<Point2<U>>, symcode_config: &SymcodeConfig) -> Option<PerspectiveTransform>
    where T: Copy + Into<f64>, U: Copy + Into<f64>
    {
        let NUM_FINDERS = finder_positions_object.len();

        if finder_positions_image.len() < NUM_FINDERS {
            console_log_util(&"Fitter error: Not enough finder candidates in this frame.");
            return None;
        }
        
        let dst_pts: Vec<PointF64> = finder_positions_object.into_iter()
            .map(|p| PointF64::new(p.x.into(), p.y.into())).collect();
        
        let check_pts: Vec<PointF64> = Self::calculate_check_points(&dst_pts, symcode_config);
        
        let mut best_transform = PerspectiveTransform::default();
        let mut min_error = std::f64::MAX;
        finder_positions_image.combination(NUM_FINDERS).for_each(|mut c| {
            c.permutation().for_each(|src_pts| {
                let src_pts: Vec<PointF64> = src_pts.iter().map(|p| PointF64::new(p.x.into(), p.y.into())).collect();
                if Self::correct_spatial_arrangement(&src_pts) {
                    let transform = PerspectiveTransform::from_point_f64(&src_pts, &dst_pts);
                    let error = Self::evaluate_transform(&transform, &src_pts, &check_pts);
                    if error < min_error {
                        best_transform = transform;
                        min_error = error;
                    }
                }
            });
        });
        if min_error > symcode_config.reprojection_error_threshold {
            None
        } else {
            Some(best_transform)
        }
    }
}