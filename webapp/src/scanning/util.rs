use visioncortex::{BinaryImage, BoundingRect, ColorHsv, ColorImage, PointF64, bound::merge_expand, color_clusters::{Cluster, Clusters, Runner}};
use wasm_bindgen::{Clamped, JsValue};
use web_sys::{ImageData, console};

use crate::canvas::Canvas;

use super::GlyphCode;

/// Check Saturation and Value in HSV
pub(crate) fn is_black(color: &ColorHsv) -> bool {
    const BLACK_LIMIT: f64 = 0.125;
    //console::log_1(&format!("{:?}", color).into());
    if color.s != 0.0 && color.v != 0.0 {
        color.s*color.v <= BLACK_LIMIT
    } else { // Either s or v is 0.0
        (if color.s > 0.0 {color.s} else {color.v}) <= BLACK_LIMIT
    }
}

pub(crate) fn color_image_to_clusters(image: ColorImage) -> Clusters {
    // Color clustering requires the use of a Runner (it is taken after run())
    let mut runner = Runner::default();
    runner.init(image);

    runner.run() // Performing clustering
}

/// Convert image to clusters then merge clusters that are close to each other
pub(crate) fn color_image_to_merged_clusters(image: ColorImage, expand_x: i32, expand_y: i32) -> Vec<(BinaryImage, BoundingRect)> {
    // Color clustering requires the use of a Runner (it is taken after run())
    let mut runner = Runner::default();
    runner.init(image.clone());

    let clusters = runner.run(); // Performing clustering
    let view = clusters.view();
    let rects: Vec<BoundingRect> =
        view.clusters_output.iter()
        .map(|&cluster_index| view.get_cluster(cluster_index).rect)
        .filter(|rect| GlyphCode::rect_not_too_large(rect))
        .collect();
    //console::log_1(&format!("{:?}", rects).into());

    let grouped_rects = merge_expand(rects, expand_x, expand_y);
    let image = image.to_binary_image(|c| is_black(&c.to_hsv()));

    grouped_rects.into_iter()
        .map(|rects| {
            let mut bounding_rect = rects[0];
            rects.into_iter().skip(1)
                .for_each(|rect| {
                    bounding_rect.merge(rect);
                });
            let cropped_image = image.crop_with_rect(bounding_rect);
            //console::log_1(&cropped_image.to_string().into());
            (cropped_image, bounding_rect)
        })
        .collect()
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
        render_bounding_rect_to_canvas(&cluster.rect, canvas);
    }
}

pub(crate) fn render_vec_image_rect_to_canvas(rects: &[(BinaryImage, BoundingRect)], canvas: &Canvas) {
    for rect in rects.iter() {
        render_bounding_rect_to_canvas(&rect.1, canvas);
    }
}

pub(crate) fn render_bounding_rect_to_canvas(rect: &BoundingRect, canvas: &Canvas) {
    let ctx = canvas.get_rendering_context_2d();
    ctx.set_stroke_style(JsValue::from_str(
        "rgb(255, 0, 0)"
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