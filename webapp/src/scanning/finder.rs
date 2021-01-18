use core::f64;

use visioncortex::{BinaryImage, Point2};

pub trait Finder {
    // Input = Image
    // Output = Vec<Point2>

    /// Returns true iff the input image has the shape of a valid finder.
    ///
    /// Note that perspective distortion has to be taken into account.
    fn shape_is_finder(image: BinaryImage) -> bool;

    /// Extract the positions of candidates of finders in the input image.
    fn extract_finder_positions<T>(image: BinaryImage) -> Vec<Point2<T>>
    where T: Into<f64> + From<i32>
    {
        let clusters = image.to_clusters(false);
        
        clusters.clusters.iter()
            .filter_map(|cluster| {
                if Self::shape_is_finder(cluster.to_binary_image()) {
                    Some(Point2::<T>::new(cluster.rect.left.into(), cluster.rect.top.into()))
                } else {
                    None
                }
            })
            .collect()
    }
}