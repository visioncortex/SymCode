use visioncortex::{BoundingRect, ColorHsv, PointI32, Shape, color_clusters::{Cluster, ClustersView}};

use crate::utils::{is_black, make_shape_square};

/// Actual Symbol or Candidate
pub(crate) struct Symbol {
    /// Average color in HSV
    pub(crate) color: ColorHsv,
    pub(crate) shape: Shape,
    /// absolute coordinates
    pub(crate) rect: BoundingRect,
    pub(crate) category: SymbolCategory,
}

pub(crate) enum SymbolCategory {
    Finder,
    Glyph,
    Invalid,
}

impl Symbol {
    pub(crate) fn from_cluster_and_view(cluster: &Cluster, view: &ClustersView) -> Self {
        let color = cluster.color().to_hsv();
        let shape = cluster.to_shape(view);
        let category = Self::categorize(&color, &shape);
        Symbol {
            color,
            shape,
            rect: cluster.rect.clone(),
            category,
        }
    }


    /// Compare this symbol with each template
    pub(crate) fn categorize(color: &ColorHsv, shape: &Shape) -> SymbolCategory {
        if !is_black(color) {
            SymbolCategory::Invalid
        } else if Self::shape_is_finder(shape) {
            SymbolCategory::Finder
        } else if Self::shape_is_glyph(shape) {
            SymbolCategory::Glyph
        } else {
            SymbolCategory::Invalid
        }
    }

    fn shape_is_finder(shape: &Shape) -> bool {
        //make_shape_square(&self.shape).is_circle()
        shape.is_circle()
    }

    fn shape_is_glyph(shape: &Shape) -> bool {
        true // All black symbols are glyphs for now
    }
}