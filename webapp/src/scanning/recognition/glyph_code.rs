use visioncortex::{PointF64, color_clusters::{Cluster, Clusters}};

use crate::{canvas::Canvas, utils::render_vec_cluster_to_canvas};

use super::GlyphCategory;

#[derive(Debug, Default)]
pub struct GlyphCode {
    glyphs: [GlyphCategory; Self::LENGTH],
}

/// Define the glyph anchors
impl GlyphCode {
    pub const CODE_WIDTH: usize = 335;
    pub const CODE_HEIGHT: usize = 195;
    
    /// Centers of the glyphs
    const ANCHORS: [PointF64; Self::LENGTH] = [
        PointF64 {
            x: 5.0,
            y: 5.0,
        },
        PointF64 {
            x: 125.0,
            y: 105.0,
        },
        PointF64 {
            x: 245.0,
            y: 5.0,
        },
    ];
}

impl GlyphCode {
    /// Square bounding box
    pub const GLYPH_SIZE: usize = 80; // 80x80

    /// True if the cluster is approximately the same size (and shape) as a valid glyph.
    ///
    /// As GLYPH_SIZE is in object space, we can define the error tolerance based on GLYPH_SIZE on an absolute scale
    pub fn cluster_size_is_reasonable(cluster: &Cluster) -> bool {
        const TOLERANCE: usize = 10; // Allows fluctuations of up to this number of units in object space

        let cluster_width = cluster.rect.width() as usize;
        let cluster_height = cluster.rect.height() as usize;

        let reasonable_error = 
            |a: usize| {(std::cmp::max(a, Self::GLYPH_SIZE) - std::cmp::min(a, Self::GLYPH_SIZE)) <= TOLERANCE};
        
        reasonable_error(cluster_width) && reasonable_error(cluster_height)
    }
}

impl GlyphCode {
    const LENGTH: usize = 3;

    /// Given clusters, for each anchor, check which cluster is the closest (and is close enough) and flag the glyph at that anchor
    pub fn add_clusters_near_anchors(&mut self, clusters: Clusters, error_threshold: f64, debug_canvas: &Canvas) {
        let view = clusters.view();
        let clusters: Vec<&Cluster> =
            view.clusters_output.iter()
                .map(|&cluster_index| view.get_cluster(cluster_index))
                .filter(|&cluster| Self::cluster_size_is_reasonable(cluster))
                .collect();
        render_vec_cluster_to_canvas(&clusters, debug_canvas);
        for (i, anchor) in Self::ANCHORS.iter().enumerate() {
            let closest_cluster = Self::find_closest_cluster(anchor, &clusters, error_threshold);
            self.set_glyph_with_cluster(i, closest_cluster);
        }
    }

    /// Find the cluster in clusters that is the closest to point, with error smaller than the error_threshold.
    fn find_closest_cluster(point: &PointF64, clusters: &[&Cluster], error_threshold: f64) -> Option<Cluster> {
        let eval_error = |p: &PointF64, c: &Cluster| {(*p - PointF64::new(c.rect.left as f64, c.rect.top as f64)).norm()};
        
        let mut closest_cluster = clusters[0];
        let mut min_error = eval_error(point, closest_cluster);
        for &cluster in clusters.iter().skip(1) {
            let error = eval_error(point, cluster);
            if error < min_error {
                closest_cluster = cluster;
                min_error = error;
            }
        }

        if min_error > error_threshold {
            None
        } else {
            Some(closest_cluster.clone())
        }
    }

    fn set_glyph_with_cluster(&mut self, i: usize, cluster: Option<Cluster>) {
        if let Some(cluster) = cluster {
            self.glyphs[i] = GlyphCategory::from_cluster(cluster);
        } // Otherwise keep it as empty
    }
}