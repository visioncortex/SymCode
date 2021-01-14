use visioncortex::{BinaryImage, BoundingRect, Color, ColorHsv, ColorImage, PointF64, PointI32, bound::merge_expand, color_clusters::{Cluster, Clusters, ClustersView, Runner, RunnerConfig}};
use wasm_bindgen::{Clamped, JsValue};
use web_sys::{ImageData, console};

use crate::{canvas::Canvas, util::console_log_util};

use super::GlyphCode;

/// Check Saturation and Value in HSV
pub(crate) fn is_black(color: &ColorHsv) -> bool {
    const BLACK_LIMIT: f64 = 0.125;
    //console_log_util(&format!("{:?}", color));
    if color.s != 0.0 && color.v != 0.0 {
        color.s*color.v <= BLACK_LIMIT
    } else { // Either s or v is 0.0
        (if color.s > 0.0 {color.s} else {color.v}) <= BLACK_LIMIT
    }
}

pub(crate) fn color_image_to_clusters(image: ColorImage) -> Clusters {
    // Color clustering requires the use of a Runner (it is taken after run())
    let runner = Runner::new(RunnerConfig {
        batch_size: 25600,
        good_min_area: 64 * 64,
        good_max_area: 256 * 256,
        is_same_color_a: 2,
        is_same_color_b: 1,
        deepen_diff: 64,
        hollow_neighbours: 1,
    }, image);

    runner.run() // Performing clustering
}

/// Convert image to clusters then merge clusters that are close to each other
pub(crate) fn color_image_to_merged_clusters(image: ColorImage, expand_x: i32, expand_y: i32) -> Vec<(BinaryImage, BoundingRect)> {
    // Color clustering requires the use of a Runner (it is taken after run())
    let mut runner = Runner::default();
    runner.init(image);

    let clusters = runner.run(); // Performing clustering
    let view = &clusters.view();
    let clusters: Vec<&Cluster> =
        view.clusters_output.iter()
        .filter_map(|&cluster_index| {
                let cluster = view.get_cluster(cluster_index);
                if GlyphCode::rect_not_too_large(&cluster.rect) {
                    Some(cluster)
                } else {
                    None
                }
        })
        .collect();

    let grouped_clusters = merge_expand(clusters, expand_x, expand_y);

    grouped_clusters.into_iter()
        .map(|clusters| {
            let mut bounding_rect = clusters[0].rect;
            clusters.iter().skip(1)
                .for_each(|cluster| {
                    bounding_rect.merge(cluster.rect);
                });
            let grouped_image = group_clusters(clusters, view, bounding_rect);
            //console_log_util(&grouped_image.to_string());
            (grouped_image, bounding_rect)
        })
        .collect()
}

fn group_clusters(clusters: Vec<&Cluster>, view: &ClustersView, overall_rect: BoundingRect) -> BinaryImage {
    let mut result = BinaryImage::new_w_h(overall_rect.width() as usize, overall_rect.height() as usize);
    let overall_offset = PointI32::new(overall_rect.left, overall_rect.top);
    clusters.into_iter()
        .for_each(|cluster| {
            let cluster_image = cluster.to_image(view);
            let offset = PointI32::new(cluster.rect.left, cluster.rect.top) - overall_offset;
            result.paste_from(&cluster_image, offset);
        });

    result
}

pub(crate) fn valid_pointf64_on_image(point: PointF64, image: &ColorImage) -> bool {
    let w_upper = (image.width - 1) as f64;
    let h_upper = (image.height - 1) as f64;

    0.0 <= point.x && point.x <= w_upper &&
    0.0 <= point.y && point.y <= h_upper
}

pub(crate) fn image_diff_area(img1: &BinaryImage, img2: &BinaryImage) -> u64 {
    img1.diff(img2).area()
}

pub(crate) fn render_color_image_to_canvas(image: &ColorImage, canvas: &Canvas) -> Result<(), JsValue> {
    let mut data = image.pixels.clone();
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), image.width as u32, image.height as u32)?;
    let ctx = canvas.get_rendering_context_2d();
    canvas.set_width(image.width);
    canvas.set_height(image.height);
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    ctx.put_image_data(&data, 0.0, 0.0)
}

pub(crate) fn render_vec_cluster_to_canvas(clusters: &[&Cluster], canvas: &Canvas) {
    for &cluster in clusters.iter() {
        render_bounding_rect_to_canvas_with_color(&cluster.rect, canvas, Color::new(0, 255, 0));
    }
}

pub(crate) fn render_vec_image_rect_to_canvas(rects: &[(BinaryImage, BoundingRect)], canvas: &Canvas) {
    for rect in rects.iter() {
        render_bounding_rect_to_canvas(&rect.1, canvas);
    }
}

pub(crate) fn render_bounding_rect_to_canvas(rect: &BoundingRect, canvas: &Canvas) {
    render_bounding_rect_to_canvas_with_color(rect, canvas, Color::new(255, 0, 0));
}

pub(crate) fn render_bounding_rect_to_canvas_with_color(rect: &BoundingRect, canvas: &Canvas, color: Color) {
    let ctx = canvas.get_rendering_context_2d();
    ctx.set_stroke_style(JsValue::from_str(
        &color.to_color_string()
        //&("rgb(".to_owned() + &color.r.to_string() + ", " + &color.g.to_string() + ", " + &color.b.to_string() + ")")
    ).as_ref());
    let x1 = rect.left as f64;
    let y1 = rect.top as f64;
    let x2 = rect.right as f64;
    let y2 = rect.bottom as f64;
    ctx.begin_path();
    ctx.move_to(x1, y1);
    ctx.line_to(x1, y2);
    ctx.line_to(x2, y2);
    ctx.line_to(x2, y1);
    ctx.line_to(x1, y1);
    ctx.stroke();
}