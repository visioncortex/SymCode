use visioncortex::{PointF64, color_clusters::{Cluster, Clusters, ClustersView}};

use crate::{canvas::Canvas, math::euclid_dist_f64, scanning::render_vec_cluster_to_canvas};

use super::{GlyphLabel, GlyphLibrary};

#[derive(Debug, Default)]
pub struct GlyphCode {
    glyphs: [GlyphLabel; Self::LENGTH],
}

/// Define the glyph anchors
impl GlyphCode {
    pub const CODE_WIDTH: usize = 400;
    pub const CODE_HEIGHT: usize = 390;
    
    /// Top-left corners of the glyphs, in U-shaped order
    const ANCHORS: [PointF64; Self::LENGTH] = [
        PointF64 {
            x: 40.0,
            y: 40.0,
        },
        PointF64 {
            x: 40.0,
            y: 160.0,
        },
        PointF64 {
            x: 160.0,
            y: 280.0,
        },
        PointF64 {
            x: 280.0,
            y: 160.0,
        },
        PointF64 {
            x: 280.0,
            y: 40.0,
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
    const LENGTH: usize = 5;

    /// Given clusters, for each anchor, check which cluster is the closest (and is close enough) and flag the glyph at that anchor
    pub fn add_clusters_near_anchors(&mut self, clusters: Clusters, error_threshold: f64, glyph_library: &GlyphLibrary, debug_canvas: &Canvas) {
        let view = clusters.view();
        let clusters: Vec<&Cluster> =
            view.clusters_output.iter()
                .map(|&cluster_index| view.get_cluster(cluster_index))
                .filter(|&cluster| Self::cluster_size_is_reasonable(cluster))
                .collect();
        render_vec_cluster_to_canvas(&clusters, debug_canvas);

        for (i, anchor) in Self::ANCHORS.iter().enumerate() {
            let closest_cluster = Self::find_closest_cluster(anchor, &clusters, error_threshold);
            self.set_glyph_with_cluster(i, closest_cluster, &view, &glyph_library);
        }
    }

    /// Find the cluster in clusters that is the closest to point, with error smaller than the error_threshold.
    fn find_closest_cluster(point: &PointF64, clusters: &[&Cluster], error_threshold: f64) -> Option<Cluster> {
        let eval_error = |p: &PointF64, c: &Cluster| {euclid_dist_f64(&p, &PointF64::new(c.rect.left as f64, c.rect.top as f64))};
        
        let (closest_cluster, min_error) =
            clusters.iter().skip(1)
            // Find the cluster with minimum error
            .fold((clusters[0], eval_error(point, clusters[0])), |min_tuple, &cluster| {
                let error = eval_error(point, cluster);
                if error < min_tuple.1 {
                    (cluster, error)
                } else {
                    min_tuple
                }
            });

        if min_error > error_threshold {
            None
        } else {
            Some(closest_cluster.clone())
        }
    }

    fn set_glyph_with_cluster(&mut self, i: usize, cluster: Option<Cluster>, view: &ClustersView, glyph_library: &GlyphLibrary) {
        if let Some(cluster) = cluster {
            self.glyphs[i] = glyph_library.find_most_similar_glyph(cluster.to_image(view));
        } else {
            self.glyphs[i] = GlyphLabel::Empty;
        }
    }
}