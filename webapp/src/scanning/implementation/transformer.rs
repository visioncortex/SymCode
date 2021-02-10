use permutator::{Combination, Permutation};
use visioncortex::{BoundingRect, PointF64};

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
    type FinderElement = BoundingRect;
    fn correct_spatial_arrangement(finder_positions_image: &[PointF64]) -> bool {
        clockwise_points_f64(&finder_positions_image[0], &finder_positions_image[1], &finder_positions_image[2]) &&
        clockwise_points_f64(&finder_positions_image[0], &finder_positions_image[3], &finder_positions_image[1]) &&
        clockwise_points_f64(&finder_positions_image[2], &finder_positions_image[1], &finder_positions_image[3])
    }

    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finders_image: Vec<&Self::FinderElement>, image_width: usize, image_height: usize, symcode_config: &SymcodeConfig) -> f64 {
        let check_points = &Self::calculate_check_points(symcode_config);

        let finder_positions_image: Vec<PointF64> = finders_image.iter().map(|finder| finder.center().to_point_f64()).collect();
        
        if finders_image.len() != check_points.len() {
            panic!("Number of finder source points and number of check points do not agree in transform evaluation.");
        }

        // The bounding box of the finder in the center (index 1 after spatial verification) should not be mapped to out of bound of object space
        let center_finder_top_left = PointF64::new(finders_image[1].left.into(), finders_image[1].top.into());
        let center_finder_top_right = PointF64::new(finders_image[1].right.into(), finders_image[1].top.into());
        let center_finder_bot_left = PointF64::new(finders_image[1].left.into(), finders_image[1].bottom.into());
        let center_finder_bot_right = PointF64::new(finders_image[1].right.into(), finders_image[1].bottom.into());
        for &point in &[center_finder_top_left, center_finder_top_right, center_finder_bot_left, center_finder_bot_right] {
            let transformed_point = img_to_obj.transform(point);
            if !valid_pointf64_on_image(transformed_point, symcode_config.code_width, symcode_config.code_height) {
                return std::f64::MAX;
            }
        }
        
        // Reproject the first check point from obj to img space
        let first_check_point_img_space = img_to_obj.transform_inverse(check_points[0]);

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

    fn fit_transform(image_width: usize, image_height: usize, finder_positions_image: Vec<Self::FinderElement>, symcode_config: &SymcodeConfig) -> Result<PerspectiveTransform, &str> {
        let dst_pts = &symcode_config.finder_positions;
        let num_finders = dst_pts.len();

        if finder_positions_image.len() < num_finders {
            return Err("Fitter error: Not enough finder candidates in this frame.");
        }
        
        let mut best_transform = Err("No spatial arrangement for the finder candidates is correct");
        let mut min_error = std::f64::MAX;
        let mut debug_min_err_src_pts: Vec<PointF64> = vec![];
        finder_positions_image.combination(num_finders).for_each(|mut c| {
            c.permutation().for_each(|src_rects| {
                let src_pts: Vec<PointF64> = src_rects.iter().map(|rect| rect.center().to_point_f64()).collect();
                if Self::correct_spatial_arrangement(&src_pts) {
                    let transform = PerspectiveTransform::from_point_f64(&src_pts, dst_pts);
                    let error = Self::evaluate_transform(&transform, src_rects, image_width, image_height, symcode_config);
                    if error < min_error {
                        best_transform = Ok(transform);
                        min_error = error;
                        debug_min_err_src_pts = src_pts;
                    }
                }
            });
        });
        debug_min_err_src_pts.into_iter().for_each(|point| {
            crate::scanning::util::render_point_i32_to_canvas_with_color(
                point.to_point_i32(),
                crate::canvas::Canvas::new_from_id("debug").as_ref().unwrap(),
                visioncortex::Color::new(0, 255, 0));
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

pub struct TransformFitterInput {
    pub finder_positions_image: Vec<BoundingRect>,
    pub raw_image_width: usize,
    pub raw_image_height: usize,
}

impl TransformFitter {
    pub fn process(input: TransformFitterInput, params: &SymcodeConfig) -> Result<PerspectiveTransform, &str> {
        // Processing starts
        Self::fit_transform(input.raw_image_width, input.raw_image_height, input.finder_positions_image, params)
    }
}