use visioncortex::{PointF64, PointI32};

use crate::{math::{PerspectiveTransform, clockwise_points_f64, euclid_dist_f64, normalize_point_f64}, scanning::{SymcodeConfig, Fitter, pipeline::ScanningProcessor}};

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

    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finder_positions_image: &[PointF64], symcode_config: &SymcodeConfig) -> f64 {
        let check_points = &Self::calculate_check_points(symcode_config);
        
        if finder_positions_image.len() != check_points.len() {
            panic!("Number of finder source points and number of check points do not agree in transform evaluation.");
        }
        // Reproject the first check point from obj to img space
        let first_check_point_img_space = img_to_obj.transform_inverse(check_points[0]);

        // Calculate the vector from the center of the first finder center to the first check point
        let first_finder_to_check_point = normalize_point_f64(&(first_check_point_img_space - finder_positions_image[0]));

        // Calculate the vectors from the centers of the remaining three finders centers
        // to the remaining check points and Calculate their errors with the above vector
        let mut acc_error = 0.0;
        finder_positions_image.iter().enumerate().skip(1).for_each(|(i, &finder_src_pt)| {
            let check_point_img_space = img_to_obj.transform_inverse(check_points[i]);
            let finder_to_check_point = normalize_point_f64(&(check_point_img_space - finder_src_pt));
            acc_error += euclid_dist_f64(&first_finder_to_check_point, &finder_to_check_point);
        });

        // Return the sum of the norms of the errors
        acc_error
    }
}

pub struct TransformFitterInput {
    pub finder_positions_image: Vec<PointI32>,
    pub raw_image_width: usize,
    pub raw_image_height: usize,
}

impl ScanningProcessor for TransformFitter {
    type Input = TransformFitterInput;

    type Output = PerspectiveTransform;

    type Params = SymcodeConfig;

    fn process(input: Self::Input, params: &Option<Self::Params>) -> Result<Self::Output, &str> {
        // Validates input and params
        Self::valid_input(&input)?;

        let params = match params {
            Some(params) => params,
            None => {return Err("Transformer Processor expects params!");}
        };

        Self::valid_params(params)?;

        // Processing starts
        Self::fit_transform(input.raw_image_width, input.raw_image_height, input.finder_positions_image, params)
    }
}