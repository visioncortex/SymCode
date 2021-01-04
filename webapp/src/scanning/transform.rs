use permutator::{Combination, Permutation};
use visioncortex::PointF64;

use crate::{math::PerspectiveTransform, scanning::ScanResult};

use super::Symbol;

pub(crate) struct Transformer {
    /// This transform is used to transform points in the raw frame to a front-facing code image
    transform: PerspectiveTransform,
}

impl Transformer {
    // In order from left to right, the top-left corner of the 3 circles and the bot-right corner of the left-most circle
    const DST_PTS: [PointF64; 4] = [PointF64 {x: 0.0, y: 10.0}, PointF64 {x: 12.0, y: 0.0}, PointF64 {x: 24.0, y: 10.0}, PointF64 {x: 8.0, y: 18.0}];

    /// If the fitting fails, None is returned.
    pub(crate) fn from_scan_result(scan_result: ScanResult) -> Option<Self> {
        let mut transform = PerspectiveTransform::default();
        let finders = &scan_result.finders;
        finders.combination(3).for_each(|mut c| {
            c.permutation().for_each(|p| {
                let src_pts = Self::get_src_pts(&p);
                transform = PerspectiveTransform::from_point_f64(&src_pts, &Self::DST_PTS);
            });
        });
        Some(Self {
            transform: PerspectiveTransform::default(),
        })
    }

    /// Given three finder candidates from the raw frame (in order from left to right), obtain the 4 reference points (check definition of Self::DST_PTS)
    fn get_src_pts(finders: &[&Symbol]) -> Vec<PointF64> {
        if finders.len() != 3 {
            panic!("Don't know how to get source points from".to_owned() + &finders.len().to_string() + "finders.")
        }

        vec![
            PointF64 {
                x: finders[0].rect.left as f64,
                y: finders[0].rect.top as f64,
            },
            PointF64 {
                x: finders[1].rect.left as f64,
                y: finders[1].rect.top as f64,
            },
            PointF64 {
                x: finders[2].rect.left as f64,
                y: finders[2].rect.top as f64,
            },
            PointF64 {
                x: finders[0].rect.right as f64,
                y: finders[0].rect.bottom as f64,
            },
        ]
    }
}