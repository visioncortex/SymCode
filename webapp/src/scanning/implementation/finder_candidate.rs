use visioncortex::{BinaryImage, ColorImage, PointI32, Shape};

use crate::{scanning::{SymcodeConfig, binarize_image_util, finder::Finder, pipeline::ScanningProcessor, render_binary_image_to_canvas}, util::console_log_util};


/// Specific implementation of Finder
pub(crate) struct FinderCandidate;

impl Finder for FinderCandidate {
    fn binarize_input_image(image: &ColorImage) -> BinaryImage {
        binarize_image_util(image)
    }

    fn shape_is_finder(image: BinaryImage) -> bool {
        Shape::from(image).is_circle()
    }
}

// FinderCandidate as a pipeline component
impl ScanningProcessor for FinderCandidate {
    type Input = *const ColorImage;

    type Output = Vec<PointI32>;

    type Params = SymcodeConfig;

    fn process(input: Self::Input, params: &Option<Self::Params>) -> Result<Self::Output, &str> {
        // Validates input and params
        if !Self::valid_input(&input) {
            return Err("Invalid input in FinderCandidates.");
        }

        let params = match params {
            Some(params) => params,
            None => {return Err("FinderCandidates Processor expects params!");}
        };

        if !Self::valid_params(params) {
            return Err("Invalid params in FinderCandidates.");
        }

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
}