use visioncortex::{BinaryImage, ColorImage, PointI32, Shape};

use crate::scanning::{SymcodeConfig, binarize_image_util, finder::Finder, valid_pointf64_on_image};

/// Specific implementation of Finder
pub(crate) struct FinderCandidate;

impl FinderCandidate {
    fn shape_is_finder(image: BinaryImage) -> bool {
        let steps = 6;
        for i in 0..steps {
            let angle = i as f64 * std::f64::consts::PI / (steps as f64);
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

impl FinderCandidate {

    pub fn process(input: &ColorImage, params: &SymcodeConfig) -> Result<Vec<PointI32>, &'static str> {
        Self::valid_params(params)?;

        // Get the reference to the input raw frame
        let raw_frame = input;
        // Binarize
        let binary_raw_frame = binarize_image_util(raw_frame);
        if let Some(debug_canvas) = &params.debug_canvas {
            crate::scanning::util::render_binary_image_to_canvas(&binary_raw_frame, debug_canvas);
        }

        // Processing starts
        let finder_candidates = Self::extract_finder_positions(binary_raw_frame);
        if let Some(debug_canvas) = &params.debug_canvas {
            Self::render_finder_candidates(&finder_candidates, debug_canvas);
        }

        if finder_candidates.len() > params.max_finder_candidates() {
            Err("Too many finder candidates!")
        } else {
            Ok(finder_candidates)
        }
    }

    pub fn valid_params(params: &SymcodeConfig) -> Result<(), &'static str> {
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

    fn render_finder_candidates(finder_candidates: &[PointI32], canvas: &crate::canvas::Canvas) {
        finder_candidates.iter().for_each(|center| {
            crate::scanning::util::render_point_i32_to_canvas(*center, canvas);
        });
    }
}