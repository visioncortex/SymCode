use visioncortex::{PointF64};

use crate::{math::{PerspectiveTransform}};

use super::{Acute32SymcodeConfig, valid_pointf64_on_image};

pub trait Fitter {
    // Input = Vec<Finders>
    // Output = (rectified) BinaryImage
    type FinderElement;

    /// Given a slice of PointF64 which are the potential finder points,
    /// verify the order based on the (user-defined) spatial arrangement so that invalid arrangements are
    /// not fitted in a transform.
    ///
    /// Note that perspective distortion has to be taken into account.
    fn correct_spatial_arrangement(finder_positions_image: &[PointF64]) -> bool;

    /// Defines the metric of evaluating a transform with the potential finder points.
    /// Returns the error of the input transform, it should be the smallest when the finders are in the correct positions.
    fn evaluate_transform(img_to_obj: &PerspectiveTransform, finder_positions_image: Vec<&Self::FinderElement>, image_width: usize, image_height: usize, symcode_config: &Acute32SymcodeConfig) -> f64;

    /// Check if the 4 corners in the object space will map to out-of-bound points in the image space.
    ///
    /// Those are points that cannot be sampled.
    fn transform_to_image_out_of_bound(image_width: usize, image_height: usize, image_to_object: &PerspectiveTransform, symcode_config: &Acute32SymcodeConfig) -> bool {
        let w = (symcode_config.code_width-1) as f64;
        let h = (symcode_config.code_height-1) as f64;
        let points_to_test = [
            PointF64::new(0.0, 0.0), PointF64::new(w, 0.0),
            PointF64::new(0.0, h), PointF64::new(w, h),
        ];

        for &point in points_to_test.iter() {
            let point_in_image_space = image_to_object.transform_inverse(point);
            
            if !valid_pointf64_on_image(point_in_image_space, image_width, image_height) {
                return true;
            }
        }
        
        false
    }
    
    /// Given finder candidates positions on the image and finder positions in the object space,
    /// find the "correct" perspective transform that maps the image space to the object space.
    ///
    /// symcode_config is used to evaluate the potential transforms.
    fn fit_transform(image_width: usize, image_height: usize, finder_positions_image: Vec<Self::FinderElement>, symcode_config: &Acute32SymcodeConfig) -> Result<PerspectiveTransform, &str>;
}