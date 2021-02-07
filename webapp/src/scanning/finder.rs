use visioncortex::BinaryImage;

pub trait Finder {
    type FinderElement;

    /// Extract the positions (centers of bounding box) of candidates of finders in the input image.
    fn extract_finder_positions(image: BinaryImage) -> Vec<Self::FinderElement>;
}