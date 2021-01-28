use visioncortex::{PointF64, PointI32};

use crate::{math::{PerspectiveTransform, clockwise_points_f64, euclid_dist_f64, euclid_dist_vec_f64, normalize_point_f64, normalize_vec_f64}, scanning::{Fitter, SymcodeConfig, valid_pointf64_on_image}};

/// Implementation of Transformer
pub(crate) struct TransformFitter;

impl TransformFitter {
    /// Use the top of each finder in object space as check points
    fn calculate_check_points(symcode_config: &crate::scanning::SymcodeConfig) -> Vec<PointF64> {
        symcode_config.finder_positions.iter()
            .map(|p| PointF64::new(p.x, p.y - (symcode_config.symbol_height >> 1) as f64))
            .collect()
    }
}

impl Fitter for TransformFitter {
    fn correct_spatial_arrangement(finder_positions_image: &[PointF64]) -> bool {
        clockwise_points_f64(&finder_positions_image[0], &finder_positions_image[1], &finder_positions_image[2]) &&
        clockwise_points_f64(&finder_positions_image[0], &finder_positions_image[3], &finder_positions_image[1]) &&
        clockwise_points_f64(&finder_positions_image[2], &finder_positions_image[1], &finder_positions_image[3])
    }

    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finder_positions_image: &[PointF64], image_width: usize, image_height: usize, symcode_config: &SymcodeConfig) -> f64 {
        let check_points = &Self::calculate_check_points(symcode_config);
        
        if finder_positions_image.len() != check_points.len() {
            panic!("Number of finder source points and number of check points do not agree in transform evaluation.");
        }
        // Reproject the first check point from obj to img space
        let first_check_point_img_space = img_to_obj.transform_inverse(check_points[0]);

        // Calculate the vector from the center of the finder center to the check point
        // the third dimension represents the magnitude (distance)
        let get_vec = |finder_position: PointF64, check_point_img_space: PointF64| {
            let p1 = finder_position;
            let p2 = check_point_img_space;

            let p1_to_p2 = &(p2-p1);
            let dist = p1_to_p2.norm();
            let p1_to_p2 = normalize_point_f64(p1_to_p2);

            vec![p1_to_p2.x, p1_to_p2.y, dist]
        };

        let first_vec = normalize_vec_f64(&get_vec(finder_positions_image[0], first_check_point_img_space));

        // Calculate the vectors from the centers of the remaining three finders centers
        // to the remaining check points and Calculate their errors with the above vector
        let mut acc_error = 0.0;
        for (i, &finder_src_pt) in finder_positions_image.iter().enumerate().skip(1) {
            let check_point_img_space = img_to_obj.transform_inverse(check_points[i]);
            // Reprojected check point is not within image boundary
            if !valid_pointf64_on_image(check_point_img_space, image_width, image_height) {
                return std::f64::MAX;
            }
            let check_vec = normalize_vec_f64(&get_vec(finder_src_pt, check_point_img_space));

            acc_error += euclid_dist_vec_f64(&first_vec, &check_vec);
        }

        // Return the sum of the norms of the errors
        acc_error
    }
}

pub struct TransformFitterInput {
    pub finder_positions_image: Vec<PointI32>,
    pub raw_image_width: usize,
    pub raw_image_height: usize,
}

impl TransformFitter {
    pub fn process(input: TransformFitterInput, params: &SymcodeConfig) -> Result<PerspectiveTransform, &str> {
        // Processing starts
        Self::fit_transform(input.raw_image_width, input.raw_image_height, input.finder_positions_image, params)
    }
}