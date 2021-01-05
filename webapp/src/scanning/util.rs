use visioncortex::{ColorHsv, ColorImage, color_clusters::{Clusters, Runner}};

/// Check Saturation and Value in HSV
pub(crate) fn is_black(color: &ColorHsv) -> bool {
    const BLACK_LIMIT: f64 = 0.125;
    //console::log_1(&format!("{:?}", color).into());
    color.s*color.v <= BLACK_LIMIT
}

pub(crate) fn color_image_to_clusters(image: ColorImage) -> Clusters {
        // Color clustering requires the use of a Runner (it is taken after run())
        let mut runner = Runner::default();
        runner.init(image);

        runner.run() // Performing clustering
}