use visioncortex::{BinaryImage, ColorImage, PointF64, PointI32};

use crate::{math::{PerspectiveTransform, clockwise_points_f64, euclid_dist_f64, normalize_point_f64}, scanning::{SymcodeConfig, Transformer as TransformerInterface, binarize_image_util, pipeline::ScanningProcessor, render_color_image_to_canvas}};

/// Implementation of Transformer
pub(crate) struct Transformer;

impl TransformerInterface for Transformer {
    fn correct_spatial_arrangement(finder_positions_image: &[PointF64]) -> bool {
        clockwise_points_f64(&finder_positions_image[0], &finder_positions_image[1], &finder_positions_image[2]) &&
        clockwise_points_f64(&finder_positions_image[0], &finder_positions_image[3], &finder_positions_image[1]) &&
        clockwise_points_f64(&finder_positions_image[2], &finder_positions_image[1], &finder_positions_image[3])
    }

    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finder_src_points: &[PointF64], check_points: &[PointF64]) -> f64 {
        if finder_src_points.len() != check_points.len() {
            panic!("Number of finder source points and number of check points do not agree in transform evaluation.");
        }
        // Reproject the first check point from obj to img space
        let first_check_point_img_space = img_to_obj.transform_inverse(check_points[0]);

        // Calculate the vector from the center of the first finder center to the first check point
        let first_finder_to_check_point = normalize_point_f64(&(first_check_point_img_space - finder_src_points[0]));

        // Calculate the vectors from the centers of the remaining three finders centers
        // to the remaining check points and Calculate their errors with the above vector
        let mut acc_error = 0.0;
        finder_src_points.iter().enumerate().skip(1).for_each(|(i, &finder_src_pt)| {
            let check_point_img_space = img_to_obj.transform_inverse(check_points[i]);
            let finder_to_check_point = normalize_point_f64(&(check_point_img_space - finder_src_pt));
            acc_error += euclid_dist_f64(&first_finder_to_check_point, &finder_to_check_point);
        });

        // Return the sum of the norms of the errors
        acc_error
    }

    /// Use the top of each finder in object space as check points
    fn calculate_check_points(finder_positions_object: &[PointF64], symcode_config: &crate::scanning::SymcodeConfig) -> Vec<PointF64> {
        finder_positions_object.iter()
            .map(|p| PointF64::new(p.x, p.y - (symcode_config.symbol_height >> 1) as f64))
            .collect()
    }

    fn binarize_image(image: &visioncortex::ColorImage) -> visioncortex::BinaryImage {
        binarize_image_util(image)
    }
}

pub(crate) struct TransformerInput {
    pub raw_image: ColorImage,
    pub finder_positions_image: Vec<PointI32>,
}

impl ScanningProcessor for Transformer {
    type Input = TransformerInput;

    type Output = BinaryImage;

    type Params = SymcodeConfig;

    fn process(input: Self::Input, params: &Option<Self::Params>) -> Result<Self::Output, &str> {
        // Validates input and params
        if !Self::valid_input(&input) {
            return Err("Invalid input in Transformer.");
        }

        let params = match params {
            Some(params) => params,
            None => {return Err("Transformer Processor expects params!");}
        };

        if !Self::valid_params(params) {
            return Err("Invalid params in Transformer.");
        }

        // Processing starts
        if let Some(rectified_image) = Self::transform_image(input.raw_image, input.finder_positions_image, params) {
            // Render rectified image to debug canvas
            if let Some(debug_canvas) = &params.debug_canvas {
                match render_color_image_to_canvas(&rectified_image.to_color_image(), debug_canvas) {
                    Ok(_) => {},
                    Err(_) => {return Err("Cannot render rectified image to debug canvas.")},
                }
            }
            
            Ok(rectified_image)
        } else {
            Err("Cannot rectify image")
        }
    }
}