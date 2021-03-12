use visioncortex::{BinaryImage, Color, ColorImage, PointF64, PointI32};

pub(crate) fn binarize_image_util(color_image: &ColorImage) -> BinaryImage {
    color_image.to_binary_image(|c| is_black_rgb(&c))
}

pub(crate) fn is_black_rgb(color: &Color) -> bool {
    let r = color.r as u32;
    let g = color.g as u32;
    let b = color.b as u32;

    r*r + g*g + b*b < 3*63*63
}

pub(crate) fn valid_pointi32_on_image(point: PointI32, image_width: usize, image_height: usize) -> bool {
    let w_upper = image_width as i32;
    let h_upper = image_height as i32;

    0 <= point.x && point.x < w_upper &&
    0 <= point.y && point.y < h_upper
}

pub(crate) fn valid_pointf64_on_image(point: PointF64, image_width: usize, image_height: usize) -> bool {
    let w_upper = image_width as f64;
    let h_upper = image_height as f64;

    0.0 <= point.x && point.x < w_upper &&
    0.0 <= point.y && point.y < h_upper
}

pub(crate) fn image_diff_area(img1: &BinaryImage, img2: &BinaryImage) -> u64 {
    img1.diff(img2).area()
}
