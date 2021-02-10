use visioncortex::{PointF64, PointI32};

use crate::{math::{PerspectiveTransform, clockwise_points_f64, euclid_dist_f64, normalize_point_f64}, scanning::{Fitter, SymcodeConfig, valid_pointf64_on_image}};

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
        
        let debug_points = &[
            // PointF64::new(203.0, 218.0),
            // PointF64::new(174.0, 175.0),
            // PointF64::new(189.0, 131.0),
            // PointF64::new(108.0, 158.0),

            PointF64::new(108.0, 158.0),
            PointF64::new(174.0, 175.0),
            PointF64::new(251.0, 200.0),
            PointF64::new(239.0, 187.0),
        ];
        let are_debug_points = |points: &[PointF64]| {
            if points.len() != debug_points.len() {
                return false;
            }
            points.iter().enumerate().fold(true, |acc, (i, point)| {
                acc &&
                crate::math::f64_approximately(point.x, debug_points[i].x) &&
                crate::math::f64_approximately(point.y, debug_points[i].y)
            })
        };

        if finder_positions_image.len() != check_points.len() {
            panic!("Number of finder source points and number of check points do not agree in transform evaluation.");
        }
        // Reproject the first check point from obj to img space
        let first_check_point_img_space = img_to_obj.transform_inverse(check_points[0]);
        if are_debug_points(finder_positions_image) {
            if let Some(debug_canvas) = &symcode_config.debug_canvas {
                crate::scanning::util::render_point_i32_to_canvas_with_color(first_check_point_img_space.to_point_i32(), debug_canvas, visioncortex::Color::new(0, 0, 255))   
            }
        }

        let get_normalized_and_norm = |p1: PointF64, p2: PointF64| {
            let v = p2 - p1;
            let norm: f64 = v.norm();
            (normalize_point_f64(&v), norm)
        };

        // Calculate the vector from the center of the first finder center to the first check point
        let (first_finder_to_check_point, first_dist) = get_normalized_and_norm(finder_positions_image[0], first_check_point_img_space);

        // Calculate the vectors from the centers of the remaining three finders centers
        // to the remaining check points and Calculate their errors with the above vector
        let mut acc_dir_error = 0.0;
        let mut shortest_dist = first_dist;
        let mut longest_dist = first_dist;
        for (i, &finder_src_pt) in finder_positions_image.iter().enumerate().skip(1) {
            let check_point_img_space = img_to_obj.transform_inverse(check_points[i]);
            if are_debug_points(finder_positions_image) {
                if let Some(debug_canvas) = &symcode_config.debug_canvas {
                    crate::scanning::util::render_point_i32_to_canvas_with_color(check_point_img_space.to_point_i32(), debug_canvas, visioncortex::Color::new(0, 255, 0))   
                }
            }
            if !valid_pointf64_on_image(check_point_img_space, image_width, image_height) {
                return std::f64::MAX;
            }
            let (finder_to_check_point, dist) = get_normalized_and_norm(finder_src_pt, check_point_img_space);
            if dist < shortest_dist {
                shortest_dist = dist;
            }
            if dist > longest_dist {
                longest_dist = dist;
            }
            acc_dir_error += euclid_dist_f64(&first_finder_to_check_point, &finder_to_check_point);
        }

        (acc_dir_error / 3.0) * 0.7 + (1.0 - shortest_dist / longest_dist) * 0.3
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