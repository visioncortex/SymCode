use visioncortex::{BinaryImage, PointI32, Shape};

use crate::{canvas::{Canvas}, scanning::{SymcodeConfig, finder::Finder, pipeline::ScanningProcessor}, util::console_log_util};


/// Specific implementation of Finder
pub(crate) struct FinderCandidate;

impl Finder for FinderCandidate {
    fn shape_is_finder(image: BinaryImage) -> bool {
        Shape::from(image).is_circle()
    }
}

// FinderCandidate as a pipeline component
impl ScanningProcessor for FinderCandidate {
    type Input = BinaryImage;

    type Output = Vec<PointI32>;

    type Params = SymcodeConfig;

    type Debug = Canvas;

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

        // Processing starts
        let finder_candidates = Self::extract_finder_positions(input);
        console_log_util(&format!("Extracted {} finder candidates from raw frame.", finder_candidates.len()));

        if finder_candidates.len() > params.max_finder_candidates {
            Err("Too many finder candidates!")
        } else {
            Ok(finder_candidates)
        }
    }
}