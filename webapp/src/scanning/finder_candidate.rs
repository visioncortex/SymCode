use visioncortex::{BinaryImage, BoundingRect, Shape, clusters::{Cluster}};

use crate::{canvas::Canvas, util::console_log_util};

use super::{render_binary_image_to_canvas, render_bounding_rect_to_canvas};


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
    pub(crate) fn extract_finder_candidates(frame: BinaryImage, canvas: &Canvas, debug_canvas: &Option<Canvas>) -> Vec<Self> {

        let clusters = frame.to_clusters(false);

        if let Some(debug_canvas) = debug_canvas {
            match render_binary_image_to_canvas(&frame, debug_canvas) {
                Ok(_) => {},
                Err(_) => console_log_util("Error in rendering first stage clustering."),
            }
        }
        
        let finder_candidates: Vec<Self> = clusters.clusters.iter()
            .filter_map(|cluster| Self::from_cluster(cluster))
            .collect();
        
        Self::render_finder_candidates(canvas, &finder_candidates);

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