use visioncortex::{BinaryImage, BoundingRect, PointI32};

use crate::{canvas::Canvas, scanning::{render_bounding_rect_to_canvas}};

use super::{GlyphLabel, GlyphLibrary};

#[derive(Debug, Default)]
pub struct GlyphCode {
    glyphs: [GlyphLabel; Self::LENGTH],
}

/// Define the glyph anchors
impl GlyphCode {
    pub const CODE_WIDTH: usize = 400;
    pub const CODE_HEIGHT: usize = 400;
    pub const CODE_QUIET_WIDTH: usize = 40;
    
    /// Top-left corners of the glyphs, in U-shaped order
    const ANCHORS: [PointI32; Self::LENGTH] = [
        PointI32 {
            x: 40,
            y: 40,
        },
        PointI32 {
            x: 40,
            y: 160,
        },
        PointI32 {
            x: 160,
            y: 280,
        },
        PointI32 {
            x: 280,
            y: 160,
        },
        PointI32 {
            x: 280,
            y: 40,
        },
    ];
}

impl GlyphCode {
    /// Square bounding box
    pub const GLYPH_SIZE: usize = 80; // 80x80

    /// As GLYPH_SIZE is in object space, we can define the error tolerance based on GLYPH_SIZE on an absolute scale
    ///
    /// Allows fluctuations of up to this number of units in object space
    const GLYPH_SIZE_TOLERANCE: usize = 10;

    pub fn rect_not_too_large(rect: &BoundingRect) -> bool {
        let rect_longer_side = std::cmp::max(rect.width(), rect.height()) as usize;
        (rect_longer_side <= Self::GLYPH_SIZE) || (rect_longer_side - Self::GLYPH_SIZE <= Self::GLYPH_SIZE_TOLERANCE) 
    }
}

impl GlyphCode {
    const LENGTH: usize = 5;

    /// Given a rectified image and the size of each glyph in the object space (assumed to be squares).
    ///
    /// Assign the corresponding glyphs to each anchor.
    pub fn from_rectified_image_by_cropping(image: BinaryImage,
        symbol_size: usize, glyph_library: &GlyphLibrary,
        stat_tolerance: f64, max_encoding_difference: usize, empty_cluster_threshold: u64,
        canvas: &Option<Canvas>
    ) -> Self {
        let mut glyph_code = Self::default();
        Self::ANCHORS.iter().enumerate().for_each(|(i, anchor)| {
            let cluster = Self::crop_cluster_at_anchor(anchor, &image, symbol_size, empty_cluster_threshold, canvas);
            glyph_code.set_glyph_with_cluster(i, cluster, glyph_library, stat_tolerance, max_encoding_difference)
        });
        glyph_code
    }

    fn crop_cluster_at_anchor(anchor: &PointI32, image: &BinaryImage, symbol_size: usize, empty_cluster_threshold: u64, canvas: &Option<Canvas>) -> Option<BinaryImage> {
        let rect = BoundingRect::new_x_y_w_h(anchor.x, anchor.y, symbol_size as i32, symbol_size as i32);
        if let Some(canvas) = canvas {
            render_bounding_rect_to_canvas(&rect, canvas);
        }
        let cluster = image.crop_with_rect(rect);
        if cluster.area() <= empty_cluster_threshold {
            None
        } else {
            Some(cluster)
        }
    }

    fn set_glyph_with_cluster(&mut self, i: usize, cluster: Option<BinaryImage>,
        glyph_library: &GlyphLibrary, stat_tolerance: f64, max_encoding_difference: usize
    ) {
        if let Some(cluster) = cluster {
            self.glyphs[i] = glyph_library.find_most_similar_glyph(cluster, stat_tolerance, max_encoding_difference);
        } else {
            self.glyphs[i] = GlyphLabel::Empty;
        }
    }
}