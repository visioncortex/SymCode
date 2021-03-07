use visioncortex::{BinaryImage, BoundingRect, ColorImage, Shape};
use crate::{interfaces::Finder as FinderInterface, interfaces::Debugger};
use super::{Acute32SymcodeConfig, binarize_image_util, valid_pointf64_on_image};

/// Specific implementation of Finder symbol element
#[derive(Default)]
pub struct CircleFinder;

impl FinderInterface for CircleFinder {

    fn to_image(&self, width: usize, height: usize) -> BinaryImage {
        Shape::circle(width, height).image
    }

    fn is_finder(&self, shape: Shape) -> bool {
        let image = shape.image;
        let steps = 6;
        for i in 0..steps {
            let angle = i as f64 * std::f64::consts::FRAC_PI_2 / (steps as f64);
            let rotated_image = if i > 0 {
                image.rotate(angle).crop()
            } else {
                image.clone()
            };
            if Shape::from(rotated_image).is_ellipse() {
                return true;
            }
        }
        false
    }
}

/// Specific implementation of Finder candidates
pub struct Acute32FinderCandidate;

impl Acute32FinderCandidate {

    pub fn process(input: &ColorImage, params: &Acute32SymcodeConfig) -> Result<Vec<BoundingRect>, &'static str> {
        Self::valid_params(params)?;

        // Get the reference to the input raw frame
        let raw_frame = input;
        // Binarize
        let binary_raw_frame = binarize_image_util(raw_frame);
        params.debugger.render_binary_image_to_canvas(&binary_raw_frame);

        // Processing starts
        let finder_candidates = Self::extract_finder_positions(binary_raw_frame, &params.finder);
        Self::render_finder_candidates(params.debugger.as_ref(), &finder_candidates);

        if finder_candidates.len() > params.max_finder_candidates() {
            Err("Too many finder candidates!")
        } else {
            Ok(finder_candidates)
        }
    }

    pub fn valid_params(params: &Acute32SymcodeConfig) -> Result<(), &'static str> {
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

    fn render_finder_candidates(debugger: &dyn Debugger, finder_candidates: &[BoundingRect]) {
        finder_candidates.iter().for_each(|rect| {
            debugger.render_point_i32_to_canvas(rect.center());
        });
    }

    fn extract_finder_positions(image: BinaryImage, finder: &CircleFinder) -> Vec<BoundingRect> {
        let clusters = image.to_clusters(false);
        
        clusters.clusters.iter()
            .filter_map(|cluster| {
                if finder.is_finder(Shape::from(cluster.to_binary_image())) {
                    Some(cluster.rect)
                } else {
                    None
                }
            })
            .collect()
    }
}