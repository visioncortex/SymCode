use visioncortex::{PointI32, color_clusters::{Cluster, Clusters}};

pub enum GlyphCategory {
    Empty,
    Square,
}

impl Default for GlyphCategory {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Default)]
pub struct GlyphCode {
    glyphs: [GlyphCategory; Self::LENGTH],
}

/// Define the glyph anchors
impl GlyphCode {
    /*
    const ANCHORS: [PointI32; Self::LENGTH] = [
        PointI32 {
            x: 0,
            y: 0,
        }
    ],
    */
}

impl GlyphCode {
    const LENGTH: usize = 3;

    /// Given clusters, for each anchor, check which cluster is the closest (and is close enough) and flag the glyph at that anchor
    pub fn add_clusters_to_anchors(&mut self, clusters: Clusters) {
        let view = clusters.view();
        let clusters: Vec<&Cluster> = view.clusters_output.iter().map(|&cluster_index| view.get_cluster(cluster_index)).collect();

    }
}