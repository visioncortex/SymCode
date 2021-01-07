use permutator::{Combination, Permutation};
use visioncortex::PointF64;
use web_sys::console;

use crate::{math::PerspectiveTransform, scanning::FinderCandidate};

pub(crate) struct TransformFitter {}

impl TransformFitter {
    // In order from left to right, the top-left corner of the 3 circles and the bot-right corner of the left-most circle
    const DST_PTS: [PointF64; 4] = [PointF64 {x: 5.0, y: 105.0}, PointF64 {x: 125.0, y: 5.0}, PointF64 {x: 245.0, y: 105.0}, PointF64 {x: 85.0, y: 185.0}];

    /// The check points are defined as the bottom-right corners of the middle and the right finders (in this order)
    const CHECK_PTS: [PointF64; 2] = [PointF64 {x: 205.0, y: 85.0}, PointF64 {x: 325.0, y: 185.0}];

    /// If the fitting fails (best_error > threshold), None is returned.
    pub(crate) fn from_finder_candidates(finder_candidates: Vec<FinderCandidate>, error_threshold: f64) -> Option<PerspectiveTransform> {
        // Need at least 3 finder candidates to try
        if finder_candidates.len() < 3 {
            console::log_1(&"Fitter error: Not enough finder candidates in this frame.".into());
            return None;
        }

        let mut best_transform = PerspectiveTransform::default();
        let mut best_error = std::f64::MAX;
        finder_candidates.combination(3).for_each(|mut c| {
            c.permutation().for_each(|p| {
                let src_pts = Self::get_src_pts(&p);
                //console::log_1(&format!("\n{:?}", src_pts).into());
                let transform = PerspectiveTransform::from_point_f64(&src_pts, &Self::DST_PTS);
                let error = Self::evaluate_transform(&transform, &p, &Self::CHECK_PTS);
                //console::log_1(&(transform.print_coeffs() + " " + &error.to_string()).into());
                if error < best_error {
                    best_transform = transform;
                    best_error = error;
                }
            });
        });
        if best_error > error_threshold { // The lowest error is not low enough
            return None;
        }
        Some(
            best_transform
        )
    }

    /// Given three finder candidates from the raw frame (in order from left to right), obtain the 4 reference points (check definition of Self::DST_PTS)
    fn get_src_pts(finders: &[&FinderCandidate]) -> Vec<PointF64> {
        if finders.len() != 3 {
            panic!("Don't know how to get source points from".to_owned() + &finders.len().to_string() + "finders.")
        }

        vec![
            PointF64::new(
                finders[0].rect.left as f64,
                finders[0].rect.top as f64,
            ),
            PointF64::new(
                finders[1].rect.left as f64,
                finders[1].rect.top as f64,
            ),
            PointF64::new(
                finders[2].rect.left as f64,
                finders[2].rect.top as f64,
            ),
            PointF64::new(
                finders[0].rect.right as f64,
                finders[0].rect.bottom as f64,
            ),
        ]
    }

    /// Given the transform candidate and the same three finder candidates as above, check whether the corresponding points in the finders agree with the check_pts.
    ///
    /// Returns the average of L2-norms of the two errors.
    fn evaluate_transform(transform: &PerspectiveTransform, finders: &[&FinderCandidate], check_pts: &[PointF64]) -> f64 {
        // Get the two points in image space
        let middle = PointF64::new(finders[1].rect.right as f64, finders[1].rect.bottom as f64);
        let right = PointF64::new(finders[2].rect.right as f64, finders[2].rect.bottom as f64);

        // Transform the two points to object space
        let middle = transform.transform(middle);
        let right = transform.transform(right);
        
        ((middle-check_pts[0]).norm() + ((right-check_pts[1]).norm())) / 2.0
    }
}