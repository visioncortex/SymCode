use visioncortex::{BinaryImage, ColorImage, PointF64, PointI32, bilinear_interpolate};

use crate::{math::PerspectiveTransform, util::console_log_util};

use super::{SymcodeConfig, is_black_rgb, render_binary_image_to_canvas};

pub trait GlyphReader {
    // Input = BinaryImage, Library
    // Output = Option<Vec<Label>>

    type Label;

    type Library;

    /// Assuming the perspective transform done in the previous stage is accurate,
    /// the areas at the anchors should be where the glyphs are.
    fn crop_at_anchor(anchor: &PointF64, image: &ColorImage, image_to_object: &PerspectiveTransform, symcode_config: &SymcodeConfig) -> Option<BinaryImage> {
        let width = symcode_config.symbol_width;
        let height = symcode_config.symbol_height;
        let mut crop = BinaryImage::new_w_h(width, height);
        for y in 0..height {
            for x in 0..width {
                let sample_point = image_to_object.transform_inverse(*anchor + PointF64::new(x as f64, y as f64)).to_point_f32();
                let interpolated_color = bilinear_interpolate(image, sample_point);
                crop.set_pixel(x, y, is_black_rgb(&interpolated_color));
            }
        }
        if crop.area() <= symcode_config.absolute_empty_cluster_threshold() {
            None
        } else {
            Some(crop)
        }
    }

    /// Finds the most similar glyph in the library based on given params
    fn find_most_similar_glyph(image: BinaryImage, glyph_library: &Self::Library, symcode_config: &crate::scanning::SymcodeConfig) -> Self::Label;

    /// Read all glyphs at the anchors on the input image
    fn read_glyphs_from_raw_frame(image: ColorImage, image_to_object: PerspectiveTransform, glyph_library: &Self::Library, symcode_config: &crate::scanning::SymcodeConfig) -> Vec<Option<Self::Label>> {
        const DEBUG_OFFSET: usize = 20;
        let mut debug_code_image = BinaryImage::new_w_h(DEBUG_OFFSET + (symcode_config.symbol_width + DEBUG_OFFSET) * symcode_config.glyph_anchors.len(), symcode_config.symbol_height + 2*DEBUG_OFFSET);
        let result = symcode_config.glyph_anchors.iter().enumerate()
            .map(|(i, anchor)| {
                let crop = Self::crop_at_anchor(anchor, &image, &image_to_object, symcode_config)?;
                debug_code_image.paste_from(&crop, PointI32::new((DEBUG_OFFSET + i*(symcode_config.symbol_width + DEBUG_OFFSET)) as i32, DEBUG_OFFSET as i32));
                Some(Self::find_most_similar_glyph(crop, glyph_library, symcode_config))
            })
            .collect();

        if let Some(debug_canvas) = &symcode_config.debug_canvas {
            if render_binary_image_to_canvas(&debug_code_image, debug_canvas).is_err() {
                console_log_util("Cannot render debug code image.");
            }
        }
        
        result
    }
}