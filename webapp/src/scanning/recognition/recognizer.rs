use std::u64;

use visioncortex::{BinaryImage, ColorImage, PointI32, color_clusters::{Cluster, ClustersView, Runner, RunnerConfig}};

use crate::{canvas::Canvas, scanning::{is_black_rgb}};

use super::{GlyphCode, GlyphLibrary};

/// Takes a rectified code image (assumed to be valid), recognizes the glyphs on it
pub struct Recognizer {}

impl Recognizer {
    pub fn recognize_glyphs_on_image(image: ColorImage,
            glyph_library: &GlyphLibrary,
            stat_tolerance: f64, max_encoding_difference: usize, empty_cluster_threshold: u64,
            debug_canvas: &Option<Canvas>) -> GlyphCode {
        let image = Self::binarize_image_by_clustering(image);
        GlyphCode::from_rectified_image_by_cropping(
            image,
            GlyphCode::GLYPH_SIZE,
            glyph_library,
            stat_tolerance,
            max_encoding_difference,
            empty_cluster_threshold,
            debug_canvas
        )
    }

    fn binarize_image_by_clustering(image: ColorImage) -> BinaryImage {
        // Color clustering requires the use of a Runner (it is taken after run())
        let runner = Runner::new(RunnerConfig {
            batch_size: 25600,
            good_min_area: 16 * 16,
            good_max_area: 256 * 256,
            is_same_color_a: 2,
            is_same_color_b: 1,
            deepen_diff: 64,
            hollow_neighbours: 1,
        }, image.clone());

        let cluster_in_quiet_zone = |cluster: &Cluster| {
            let center = cluster.rect.center();
            let quiet_width = GlyphCode::CODE_QUIET_WIDTH as i32;
            let code_width = GlyphCode::CODE_WIDTH as i32;
            let code_height = GlyphCode::CODE_HEIGHT as i32;
            center.x < quiet_width ||
            center.y < quiet_width ||
            center.x > code_width - quiet_width ||
            center.y > code_height - quiet_width
        };

        let clusters = runner.run(); // Performing clustering
        let view = &clusters.view();

        // Select valid clusters
        let clusters: Vec<&Cluster> = 
            view.clusters_output.iter()
            .filter_map(|&cluster_index| {
                let cluster = view.get_cluster(cluster_index);
                if GlyphCode::rect_not_too_large(&cluster.rect) &&
                    is_black_rgb(&cluster.color()) &&
                    !cluster_in_quiet_zone(cluster)
                {
                    Some(cluster)
                } else {
                    None
                }
            })
            .collect();

        let mut result = BinaryImage::new_w_h(image.width, image.height);
        Self::plot_clusters_onto_binary_image(&mut result, clusters, view);
        result
    }

    fn plot_clusters_onto_binary_image(image: &mut BinaryImage, clusters: Vec<&Cluster>, view: &ClustersView) {
        clusters.into_iter().for_each(|cluster| {
            let cluster_image = cluster.to_image(view);
            let offset = PointI32::new(cluster.rect.left, cluster.rect.top);
            image.paste_from(&cluster_image, offset);
        });
    }
}