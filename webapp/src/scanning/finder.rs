use visioncortex::{BinaryImage, ColorImage, PointI32};

pub trait Finder {
    // Input = Image
    // Output = Vec<Point2>

    /// Binarize the input image so that binary clustering can be applied on it.
    fn binarize_input_image(image: &ColorImage) -> BinaryImage;
    
    /// Returns true iff the input image has the shape of a valid finder.
    ///
    /// Note that perspective distortion has to be taken into account.
    fn shape_is_finder(image: BinaryImage) -> bool;

    /// Extract the positions (centers of bounding box) of candidates of finders in the input image.
    fn extract_finder_positions(image: BinaryImage) -> Vec<PointI32>
    {
        let clusters = image.to_clusters(false);
        
        clusters.clusters.iter()
            .filter_map(|cluster| {
                if Self::shape_is_finder(cluster.to_binary_image()) {
                    Some(cluster.rect.center())
                } else {
                    None
                }
            })
            .collect()
    }
}