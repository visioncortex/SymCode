use std::u64;

use visioncortex::{BinaryImage, BoundingRect, PointF64};

use super::{SymcodeConfig, render_bounding_rect_to_canvas};

pub trait GlyphReader {
    // Input = BinaryImage
    // Output = Option<Vec<Glyph>>

    type Label;

    type Library;

    fn crop_at_anchor(anchor: &PointF64, image: &BinaryImage, symcode_config: &SymcodeConfig, absolute_empty_cluster_threshold: u64) -> Option<BinaryImage> {
        let rect = BoundingRect::new_x_y_w_h(anchor.x as i32, anchor.y as i32, symcode_config.symbol_width as i32, symcode_config.symbol_height as i32);
        if let Some(canvas) = symcode_config.debug_canvas {
            render_bounding_rect_to_canvas(&rect, canvas);
        }
        let cluster = image.crop_with_rect(rect);
        if cluster.area() <= absolute_empty_cluster_threshold {
            None
        } else {
            Some(cluster)
        }
    }

    /// Finds the most similar glyph in the library based on given params
    fn find_most_similar_glyph(image: BinaryImage, glyph_library: &Self::Library, symcode_config: &crate::scanning::SymcodeConfig) -> Self::Label;

    /// Read all glyphs at the anchors on the input image
    fn read_glyphs_from_rectified_image(image: visioncortex::BinaryImage, glyph_library: &Self::Library, symcode_config: &crate::scanning::SymcodeConfig) -> Vec<Option<Self::Label>> {
        let absolute_empty_cluster_threshold = (symcode_config.empty_cluster_threshold * (symcode_config.symbol_width * symcode_config.symbol_height) as f64) as u64;
        symcode_config.glyph_anchors.iter()
            .map(|anchor| {
                let crop = Self::crop_at_anchor(anchor, &image, symcode_config, absolute_empty_cluster_threshold)?;
                Some(Self::find_most_similar_glyph(crop, glyph_library, symcode_config))
            })
            .collect()
    }
}