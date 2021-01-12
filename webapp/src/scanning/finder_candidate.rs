use visioncortex::{BoundingRect, ColorHsv, ColorImage, Shape, color_clusters::{Cluster, ClustersView}};

use crate::{canvas::Canvas, util::console_log_util};

use super::{color_image_to_clusters, is_black, render_bounding_rect_to_canvas, render_color_image_to_canvas};


/// Those whose color and shape are close to an actual Finder
pub(crate) struct FinderCandidate {
    /// absolute coordinates
    pub(crate) rect: BoundingRect,
}

/// Operates for/on a collection of FinderCandidate
impl FinderCandidate {
    /// Extract the Finder patterns.
    ///
    /// Decision is made based on the colors and shapes of each cluster.
    pub(crate) fn extract_finder_candidates(frame: &ColorImage, canvas: &Canvas, debug_canvas: &Canvas) -> Vec<Self> {

        let clusters = color_image_to_clusters(frame.clone());
        let view = clusters.view();

        match render_color_image_to_canvas(&view.to_color_image(), debug_canvas) {
            Ok(_) => {},
            Err(_) => console_log_util("Error in rendering first stage clustering."),
        }
        
        let finder_candidates: Vec<Self> = view.clusters_output.iter()
            .filter_map(|&cluster_index| Self::from_cluster_and_view(view.get_cluster(cluster_index), &view))
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
    pub(crate) fn from_cluster_and_view(cluster: &Cluster, view: &ClustersView) -> Option<Self> {
        let color = cluster.color().to_hsv();
        let shape = cluster.to_shape(view);
        if Self::is_finder(&shape, &color) {
            Some (
                FinderCandidate {
                    rect: cluster.rect,
                }
            )
        } else {
            None
        }
    }

    fn is_finder(shape: &Shape, color: &ColorHsv) -> bool {
        is_black(color) && Self::shape_is_finder(shape)
    }

    fn shape_is_finder(shape: &Shape) -> bool {
        //make_shape_square(&self.shape).is_circle()
        shape.is_circle()
    }
}