use visioncortex::{BinaryImage, ColorImage, PointI32, Shape};

use crate::{scanning::{SymcodeConfig, binarize_image_util, finder::Finder, pipeline::ScanningProcessor, valid_pointf64_on_image}};

/// Specific implementation of Finder
pub(crate) struct FinderCandidate;

impl FinderCandidate {
    fn shape_is_finder(image: BinaryImage) -> bool {
        for i in 0..6 {
            let angle = i as f64 * std::f64::consts::PI / 6.0;
            let rotated_image = if i > 0 {
                image.rotate(angle)
            } else {
                image.clone()
            };
            if Shape::from(rotated_image).is_circle() {
                return true;
            }
        }
        false
    }
}

impl Finder for FinderCandidate {
    type FrameInput = BinaryImage;

    type FinderElement = PointI32;

    fn extract_finder_positions(image: Self::FrameInput) -> Vec<Self::FinderElement> {
        let clusters = image.to_clusters(false);
        
        clusters.clusters.iter()
            .filter_map(|cluster| {
                if Self::shape_is_finder(cluster.to_binary_image()) {
                    Some(cluster.rect.center())
                } else {
                    None
                }
            })
            .collect()
    }
}

// FinderCandidate as a pipeline component
impl ScanningProcessor for FinderCandidate {
    type Input = *const ColorImage;

    type Output = Vec<PointI32>;

    type Params = SymcodeConfig;

    fn process(input: Self::Input, params: &Self::Params) -> Result<Self::Output, &str> {
        Self::valid_input_and_params(&input, params)?;

        // Get the reference to the input raw frame
        let raw_frame = unsafe {&*input};
        // Binarize
        let binary_raw_frame = binarize_image_util(raw_frame);

        // Processing starts
        let finder_candidates = Self::extract_finder_positions(binary_raw_frame);
        //console_log_util(&format!("Extracted {} finder candidates from raw frame.", finder_candidates.len()));

        if finder_candidates.len() > params.max_finder_candidates() {
            Err("Too many finder candidates!")
        } else {
            Ok(finder_candidates)
        }
    }

    fn valid_input_and_params(input: &Self::Input, params: &Self::Params) -> Result<(), &'static str> {
        if params.finder_positions.len() < 4 {
            return Err("Number of finder candidates specified in FinderCandidates' params is less than 4.");
        }

        // Each finder position cannot be out of boundary of the code
        for &finder in params.finder_positions.iter() {
            if !valid_pointf64_on_image(finder, params.code_width, params.code_height) {
                return Err("A finder is out of the boundary in the object space.");
            }
        }

        Ok(())
    }
}