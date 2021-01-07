use visioncortex::color_clusters::Cluster;

#[derive(Debug)]
pub enum GlyphCategory {
    Empty,
    Square,
}

impl Default for GlyphCategory {
    fn default() -> Self {
        Self::Empty
    }
}

impl GlyphCategory {
    // TODO
    pub fn from_cluster(cluster: Cluster) -> Self {
        Self::Square
    }
}