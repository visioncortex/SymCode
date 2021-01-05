use visioncortex::{BoundingRect, ColorHsv, Shape, color_clusters::{Cluster, ClustersView}};

use super::is_black;


/// Those whose color and shape are close to an actual Finder
pub(crate) struct FinderCandidate {
    pub(crate) shape: Shape,
    /// absolute coordinates
    pub(crate) rect: BoundingRect,
}

impl FinderCandidate {
    pub(crate) fn from_cluster_and_view(cluster: &Cluster, view: &ClustersView) -> Option<Self> {
        let color = cluster.color().to_hsv();
        let shape = cluster.to_shape(view);
        if Self::is_finder(&shape, &color) {
            Some (
                FinderCandidate {
                    shape,
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