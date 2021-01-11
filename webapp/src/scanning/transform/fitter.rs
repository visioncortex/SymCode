use permutator::{Combination, Permutation};
use visioncortex::PointF64;
use web_sys::console;

use crate::{math::{PerspectiveTransform, clockwise_points_f64, euclid_dist_f64}, scanning::FinderCandidate};



pub(crate) struct TransformFitter {}

impl TransformFitter {
    const NUM_FINDERS: usize = 4;

    // In order from top to bottom, left to right, the centers of the 4 circles
    const DST_PTS: [PointF64; 4] = [PointF64 {x: 200.0, y: 80.0}, PointF64 {x: 200.0, y: 200.0}, PointF64 {x: 80.0, y: 320.0}, PointF64 {x: 320.0, y: 320.0}];

    /// The check points are defined as the top of the circles in the same order as above
    const CHECK_PTS: [PointF64; 4] = [PointF64 {x: 200.0, y: 40.0}, PointF64 {x: 200.0, y: 160.0}, PointF64 {x: 80.0, y: 280.0}, PointF64 {x: 320.0, y: 280.0}];

    /// If the fitting fails (best_error > threshold), None is returned.
    pub(crate) fn from_finder_candidates(finder_candidates: Vec<FinderCandidate>, error_threshold: f64) -> Option<PerspectiveTransform> {
        // Need at least 4 finder candidates to try
        if finder_candidates.len() < Self::NUM_FINDERS {
            console::log_1(&"Fitter error: Not enough finder candidates in this frame.".into());
            return None;
        }

        let mut best_transform = PerspectiveTransform::default();
        let mut min_error = std::f64::MAX;
        finder_candidates.combination(4).for_each(|mut c| {
            c.permutation().for_each(|p| {
                if Self::correct_spatial_arrangement(&p) {
                    let src_pts = Self::get_src_pts(&p);
                    //console::log_1(&format!("\n{:?}", src_pts).into());
                    let transform = PerspectiveTransform::from_point_f64(&src_pts, &Self::DST_PTS);
                    let error = Self::evaluate_transform(&transform, &src_pts);
                    //console::log_1(&(transform.print_coeffs() + " " + &error.to_string()).into());
                    if error < min_error {
                        best_transform = transform;
                        min_error = error;
                        // console::log_1(&format!("\n{:?}", src_pts).into());
                        // console::log_1(&min_error.into());
                    }
                }
            });
        });
        if min_error > error_threshold { // The lowest error is not low enough
            console::log_1(&format!("Fitter error: min fitting error {} > error threshold {}.", min_error, error_threshold).into());
            return None;
        }
        Some(
            best_transform
        )
    }

    fn correct_spatial_arrangement(candidates: &[&FinderCandidate]) -> bool {
        let position = |candidate: &FinderCandidate| { PointF64::new(candidate.rect.left as f64, candidate.rect.top as f64) };
        let positions: Vec<PointF64> = candidates.iter().map(|&candidate| position(candidate)).collect();

        // This is specific to the current design
        clockwise_points_f64(&positions[0], &positions[1], &positions[2]) &&
        clockwise_points_f64(&positions[0], &positions[3], &positions[1]) &&
        clockwise_points_f64(&positions[2], &positions[1], &positions[3])
    }

    /// Given four finder candidates from the raw frame (in order from left to right), obtain the 4 reference points (check definition of Self::DST_PTS)
    fn get_src_pts(finders: &[&FinderCandidate]) -> Vec<PointF64> {
        if finders.len() != Self::NUM_FINDERS {
            panic!("Don't know how to get source points from ".to_owned() + &finders.len().to_string() + " finders.")
        }

        vec![
            finders[0].rect.center().to_point_f64(),
            finders[1].rect.center().to_point_f64(),
            finders[2].rect.center().to_point_f64(),
            finders[3].rect.center().to_point_f64(),
        ]
    }

    /// Test if the four candidates are in the same order as the ones in the object space
    ///
    /// Returns the sum of L2-norms of the three errors.
    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finder_src_pts: &[PointF64]) -> f64 {
        // Reproject the first check point from obj to img space
        let first_check_point_img_space = img_to_obj.transform_inverse(Self::CHECK_PTS[0]);

        // Calculate the vector from the center of the first finder center to the first check point
        let first_finder_to_check_point = first_check_point_img_space - finder_src_pts[0];

        // Calculate the vectors from the centers of the remaining three finders centers
        // to the remaining check points and Calculate their errors with the above vector
        let mut acc_error = 0.0;
        finder_src_pts.iter().enumerate().skip(1).for_each(|(i, &finder_src_pt)| {
            let check_point_img_space = img_to_obj.transform_inverse(Self::CHECK_PTS[i]);
            let finder_to_check_point = check_point_img_space - finder_src_pt;
            acc_error += euclid_dist_f64(&first_finder_to_check_point, &finder_to_check_point);
        });

        // Return the sum of the norms of the errors
        acc_error
    }
}