use permutator::{Combination, Permutation};
use visioncortex::{PointF64, PointI32};

use crate::{math::{PerspectiveTransform, clockwise_points_f64, euclid_dist_f64, normalize_point_f64}, scanning::FinderCandidate, util::console_log_util};



pub(crate) struct TransformFitter {}

impl TransformFitter {
    const NUM_FINDERS: usize = 4;

    // In order from top to bottom, left to right, the centers of the 4 circles
    const DST_PTS: [PointF64; 4] = [PointF64 {x: 200.0, y: 80.0}, PointF64 {x: 200.0, y: 200.0}, PointF64 {x: 80.0, y: 320.0}, PointF64 {x: 320.0, y: 320.0}];

    /// The check points are defined as the top of the circles in the same order as above
    const CHECK_PTS: [PointF64; 4] = [PointF64 {x: 200.0, y: 40.0}, PointF64 {x: 200.0, y: 160.0}, PointF64 {x: 80.0, y: 280.0}, PointF64 {x: 320.0, y: 280.0}];

    /// If the fitting fails (best_error > threshold), None is returned.
    pub(crate) fn from_finder_candidates(finder_candidates: Vec<PointI32>, error_threshold: f64) -> Option<PerspectiveTransform> {
        // Need at least 4 finder candidates to try
        if finder_candidates.len() < Self::NUM_FINDERS {
            console_log_util(&"Fitter error: Not enough finder candidates in this frame.");
            return None;
        }

        let mut best_transform = PerspectiveTransform::default();
        let mut min_error = std::f64::MAX;
        finder_candidates.combination(4).for_each(|mut c| {
            c.permutation().for_each(|src_pts| {
                let src_pts: Vec<PointF64> = src_pts.iter().map(|p| PointF64::new(p.x as f64, p.y as f64)).collect();
                if Self::correct_spatial_arrangement(&src_pts) {
                    //console_log_util(&format!("\n{:?}", src_pts));
                    let transform = PerspectiveTransform::from_point_f64(&src_pts, &Self::DST_PTS);
                    let error = Self::evaluate_transform(&transform, &src_pts);
                    //console_log_util(&(transform.print_coeffs() + " " + &error.to_string()));
                    if error < min_error {
                        best_transform = transform;
                        min_error = error;
                        // console_log_util(&format!("\n{:?}", src_pts));
                        // console_log_util(&min_error.to_string());
                    }
                }
            });
        });
        if min_error > error_threshold { // The lowest error is not low enough
            console_log_util(&format!("Fitter error: min fitting error {} > error threshold {}.", min_error, error_threshold));
            return None;
        }
        Some(
            best_transform
        )
    }

    fn correct_spatial_arrangement(candidates: &[PointF64]) -> bool {
        // This is specific to the current design
        clockwise_points_f64(&candidates[0], &candidates[1], &candidates[2]) &&
        clockwise_points_f64(&candidates[0], &candidates[3], &candidates[1]) &&
        clockwise_points_f64(&candidates[2], &candidates[1], &candidates[3])
    }

    /// Test if the four candidates are in the same order as the ones in the object space
    ///
    /// Returns the sum of L2-norms of the three errors.
    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finder_src_pts: &[PointF64]) -> f64 {
        // Reproject the first check point from obj to img space
        let first_check_point_img_space = img_to_obj.transform_inverse(Self::CHECK_PTS[0]);

        // Calculate the vector from the center of the first finder center to the first check point
        let first_finder_to_check_point = normalize_point_f64(&(first_check_point_img_space - finder_src_pts[0]));

        // Calculate the vectors from the centers of the remaining three finders centers
        // to the remaining check points and Calculate their errors with the above vector
        let mut acc_error = 0.0;
        finder_src_pts.iter().enumerate().skip(1).for_each(|(i, &finder_src_pt)| {
            let check_point_img_space = img_to_obj.transform_inverse(Self::CHECK_PTS[i]);
            let finder_to_check_point = normalize_point_f64(&(check_point_img_space - finder_src_pt));
            acc_error += euclid_dist_f64(&first_finder_to_check_point, &finder_to_check_point);
        });

        // Return the sum of the norms of the errors
        acc_error
    }
}