pub trait Finder {
    type FrameInput;
    type FinderElement;

    /// Extract the positions (centers of bounding box) of candidates of finders in the input image.
    fn extract_finder_positions(image: Self::FrameInput) -> Vec<Self::FinderElement>;
}