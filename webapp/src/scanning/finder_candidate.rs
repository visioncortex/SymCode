use visioncortex::{BinaryImage, BoundingRect, Shape, clusters::{Cluster}};

use crate::{canvas::{Canvas}, util::console_log_util};

use super::{pipeline::ScanningProcessor, render_binary_image_to_canvas, render_bounding_rect_to_canvas};


/// Those whose color and shape are close to an actual Finder
pub(crate) struct FinderCandidate {
    /// absolute coordinates
    pub(crate) rect: BoundingRect,
}

/// Operates for/on a collection of FinderCandidate
impl FinderCandidate {
    /// Extract the Finder patterns.
    ///
    /// Decision is made based on the shapes of each cluster.
    pub(crate) fn extract_finder_candidates(frame: BinaryImage, canvas: &Option<Canvas>) -> Vec<Self> {

        let clusters = frame.to_clusters(false);
        
        let finder_candidates: Vec<Self> = clusters.clusters.iter()
            .filter_map(|cluster| Self::from_cluster(cluster))
            .collect();
        
        if let Some(canvas) = canvas {
            Self::render_finder_candidates(canvas, &finder_candidates);
        }

        finder_candidates
    }

    fn render_finder_candidates(canvas: &Canvas, finder_candidates: &[Self]) {
        for finder in finder_candidates.iter() {
            let rect = &finder.rect;
            render_bounding_rect_to_canvas(rect, canvas);
        }
    }
}

/// Operates for/on a single FinderCandidate
impl FinderCandidate {
    pub(crate) fn from_cluster(cluster: &Cluster) -> Option<Self> {
        let shape = Shape::from(cluster.to_binary_image());
        if Self::shape_is_finder(shape) {
            Some (
                FinderCandidate {
                    rect: cluster.rect,
                }
            )
        } else {
            None
        }
    }

    fn shape_is_finder(shape: Shape) -> bool {
        //make_shape_square(&self.shape).is_circle()
        shape.is_circle()
    }
}

impl ScanningProcessor for FinderCandidate {
    type Input = BinaryImage;

    type Output = Vec<Self>;

    type Params = bool; // None

    type Debug = Canvas;

    fn process(input: Self::Input, params: Option<Self::Params>, debug: &Option<Self::Debug>) -> Result<Self::Output, String> {
        if params.is_some() {
            console_log_util(&"FinderCandidate Processor expects no params!");
            panic!();
        }

        // Validates input and params

        if !Self::valid_input(&input) {
            return Err("Invalid input in FinderCandidates.".into());
        }

        if let Some(params) = params {
            if !Self::valid_params(&params) {
                return Err("Invalid params in FinderCandidates.".into());
            }
        }

        // Processing starts
        let finder_candidates = Self::extract_finder_candidates(input, debug);
        
        Ok(finder_candidates)
    }

    fn valid_input(input: &Self::Input) -> bool {
        true
    }

    fn valid_params(params: &Self::Params) -> bool {
        true
    }
}