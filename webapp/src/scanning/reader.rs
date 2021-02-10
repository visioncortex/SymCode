use visioncortex::{BinaryImage, BoundingRect, ColorImage, PointF64, PointI32};

use crate::{math::PerspectiveTransform};

use super::{SymcodeConfig, is_black_rgb, render_binary_image_to_canvas};

pub trait GlyphReader {
    // Input = BinaryImage, Library
    // Output = Option<Vec<Label>>

    type Label;

    type Library;

    fn rectify_image(raw_image: ColorImage, image_to_object: PerspectiveTransform, symcode_config: &SymcodeConfig) -> BinaryImage {
        let width = symcode_config.code_width;
        let height = symcode_config.code_height;
        let mut rectified_image = BinaryImage::new_w_h(width, height);
        for y in 0..height {
            for x in 0..width {
                let sample_point = image_to_object.transform_inverse(PointF64::new(x as f64, y as f64)).to_point_f32();
                let interpolated_color = match raw_image.sample_pixel_at_safe(sample_point) {
                    Some(color) => color,
                    None => visioncortex::Color::color(&visioncortex::ColorName::White),
                };
                rectified_image.set_pixel(x, y, is_black_rgb(&interpolated_color));
            }
        }
        rectified_image
    }

    /// For each rect in cluster_rects, classify it into the group of rects that overlap with the glyph region
    fn group_cluster_rects_by_glyph_regions(mut cluster_rects: Vec<BoundingRect>, symcode_config: &SymcodeConfig) -> Vec<Vec<BoundingRect>> {
        let glyph_rects: Vec<BoundingRect> = symcode_config.glyph_anchors.iter().map(|top_left| {
            BoundingRect::new_x_y_w_h(top_left.x as i32, top_left.y as i32, symcode_config.symbol_width as i32, symcode_config.symbol_height as i32)
        }).collect();

        let mut grouped_rects = vec![vec![]; glyph_rects.len()];
        glyph_rects.into_iter().enumerate().for_each(|(i, glyph_rect)| {
            cluster_rects.retain(|cluster_rect| {
                if glyph_rect.hit(*cluster_rect) {
                    grouped_rects[i].push(*cluster_rect);
                    false
                } else {
                    true
                }
            });
        });
        
        grouped_rects
    }

    /// Merge each group of cluster rects and returns the centers of the merged clusters, or None if the group has no cluster
    fn centers_of_merged_clusters_in_glyph_regions(grouped_cluster_rects: Vec<Vec<BoundingRect>>) -> Vec<Option<PointI32>> {
        grouped_cluster_rects.into_iter()
            .map(|group| {
                if group.is_empty() {
                    None
                } else {
                    let rect = group[0];
                    let merged_cluster = group.iter().skip(1).fold(rect, |mut a, b| {
                        a.merge(*b); a
                    });
                    Some(
                        merged_cluster.center()
                    )
                }
            })
            .collect()
    }

    /// Crop an image of a glyph at the specified center position
    fn crop_glyph_at_center(image: &BinaryImage, center: PointI32, symcode_config: &SymcodeConfig) -> BinaryImage {
        let width = symcode_config.symbol_width;
        let height = symcode_config.symbol_height;
        let top_left = center - PointI32::new((width >> 1) as i32, (height >> 1) as i32);
        let rect = BoundingRect::new_x_y_w_h(top_left.x, top_left.y, width as i32, height as i32);
        if let Some(debug_canvas) = &symcode_config.debug_canvas {
            crate::scanning::util::render_bounding_rect_to_canvas(&rect, debug_canvas);
        }
        image.crop_with_rect(rect)
    }

    /// Finds the most similar glyph in the library based on given params
    fn find_most_similar_glyph(image: BinaryImage, glyph_library: &Self::Library, symcode_config: &SymcodeConfig) -> Self::Label;
    
    /// Read all glyphs at the anchors on the input image
    fn read_glyphs_from_raw_frame(image: ColorImage, image_to_object: PerspectiveTransform, glyph_library: &Self::Library, symcode_config: &crate::scanning::SymcodeConfig) -> Vec<Option<Self::Label>> {
        let rectified_image = Self::rectify_image(image, image_to_object, symcode_config);
        if let Some(debug_canvas) = &symcode_config.debug_canvas {
            if render_binary_image_to_canvas(&rectified_image, debug_canvas).is_err() {
                crate::util::console_log_util("Cannot render rectified code image to debug canvas.");
            }
        }
        let cluster_rects: Vec<BoundingRect> = rectified_image.to_clusters(true).clusters.into_iter()
            .filter_map(|cluster| {
                let rect = cluster.rect;
                if cluster.size() < symcode_config.absolute_empty_cluster_threshold(rect.width() as usize, rect.height() as usize) as usize {
                    return None;
                }
                if  rect.width() <= (symcode_config.symbol_width + 10) as i32 &&
                    rect.height() <= (symcode_config.symbol_height + 10) as i32 &&
                    rect.width() >= (symcode_config.symbol_width >> 4) as i32 &&
                    rect.height() >= (symcode_config.symbol_height >> 4) as i32  {
                    Some(rect)
                } else {
                    None
                }
            })
            .collect();

        //cluster_rects.iter().for_each(|rect| crate::scanning::util::render_bounding_rect_to_canvas_with_color(rect, crate::canvas::Canvas::new_from_id("debug").as_ref().unwrap(), visioncortex::Color::new(0, 0, 255)));
        let grouped_cluster_rects = Self::group_cluster_rects_by_glyph_regions(cluster_rects, symcode_config);
        let centers_of_groups = Self::centers_of_merged_clusters_in_glyph_regions(grouped_cluster_rects);
        centers_of_groups.into_iter().map(|center| {
            if let Some(center) = center {
                let glyph_image = Self::crop_glyph_at_center(&rectified_image, center, symcode_config);
                if glyph_image.area() < symcode_config.absolute_empty_cluster_threshold(glyph_image.width, glyph_image.height) {
                    None
                } else {
                    Some(Self::find_most_similar_glyph(glyph_image, glyph_library, symcode_config))
                }
            } else {
                None
            }
        })
        .collect()
    }
}