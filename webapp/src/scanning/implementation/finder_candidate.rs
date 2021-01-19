use visioncortex::{BinaryImage, PointI32, Shape};

use crate::{canvas::{Canvas}, scanning::{finder::Finder, pipeline::ScanningProcessor}, util::console_log_util};


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

    type Params = bool; // None

    type Debug = Canvas;

    fn process(input: Self::Input, params: &Option<Self::Params>, debug: &Option<Self::Debug>) -> Result<Self::Output, String> {
        if params.is_some() {
            console_log_util(&"FinderCandidate Processor expects no params!");
            panic!();
        }

        // Validates input and params
        if !Self::valid_input(&input) {
            return Err("Invalid input in FinderCandidates.".into());
        }

        if let Some(params) = params {
            if !Self::valid_params(params) {
                return Err("Invalid params in FinderCandidates.".into());
            }
        }

        // Processing starts
        Ok(Self::extract_finder_positions(input))
    }
}