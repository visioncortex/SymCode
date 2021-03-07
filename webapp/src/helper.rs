use visioncortex::ColorHsv;

/// Check Saturation and Value in HSV
pub(crate) fn is_black_hsv(color: &ColorHsv) -> bool {
    const BLACK_LIMIT: f64 = 0.125;
    //console_log_util(&format!("{:?}", color));
    if color.s != 0.0 && color.v != 0.0 {
        color.s*color.v <= BLACK_LIMIT
    } else { // Either s or v is 0.0
        (if color.s > 0.0 {color.s} else {color.v}) <= BLACK_LIMIT
    }
}
