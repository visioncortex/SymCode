use visioncortex::{BinaryImage, ColorImage, PointI32, Shape};

use crate::{scanning::{SymcodeConfig, binarize_image_util, finder::Finder, pipeline::ScanningProcessor, render_binary_image_to_canvas}, util::console_log_util};

/// Specific implementation of Finder
pub(crate) struct FinderCandidate;

impl FinderCandidate {
    fn shape_is_finder(image: BinaryImage) -> bool {
        Shape::from(image).is_circle()
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

    fn process(input: Self::Input, params: &Option<Self::Params>) -> Result<Self::Output, &str> {
        Self::valid_input_and_params(&input, params)?;

        let params = params.as_ref().unwrap();

        // Get the reference to the input raw frame
        let raw_frame = unsafe {&*input};
        // Binarize
        let binary_raw_frame = binarize_image_util(raw_frame);
        // Take a look
        if let Some(canvas) = &params.canvas {
            render_binary_image_to_canvas(&binary_raw_frame, canvas);
        }

        // Processing starts
        let finder_candidates = Self::extract_finder_positions(binary_raw_frame);
        console_log_util(&format!("Extracted {} finder candidates from raw frame.", finder_candidates.len()));

        if finder_candidates.len() > params.max_finder_candidates() {
            Err("Too many finder candidates!")
        } else {
            Ok(finder_candidates)
        }
    }

    fn valid_input_and_params(input: &Self::Input, params: &Option<Self::Params>) -> Result<(), &'static str> {
        let params = if let Some(params) = params {params} else {
            return Err("FinderCandidates Processor expects params.");
        };

        if params.finder_positions.len() < 4 {
            return Err("Number of finder candidates specified in FinderCandidates' params is less than 4.");
        }

        // Each finder position cannot be out of 

        Ok(())
    }
}