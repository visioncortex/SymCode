use visioncortex::{ColorHsv, ColorImage, PointF64, color_clusters::{Clusters, Runner}};

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

pub(crate) fn valid_pointf64_on_image(point: PointF64, image: &ColorImage) -> bool {
    let w_upper = (image.width - 1) as f64;
    let h_upper = (image.height - 1) as f64;

    0.0 <= point.x && point.x <= w_upper &&
    0.0 <= point.y && point.y <= h_upper
}