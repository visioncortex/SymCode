use visioncortex::{BinaryImage, Color, ColorImage, PointF64, PointI32, SampleStatBuilder, SummedAreaTable};

// Local adaptive thresholding by finding patch mean around each pixel
pub(crate) fn binarize_image_util(color_image: &ColorImage, patch_size: usize, offset: i32) -> BinaryImage {
    let sat = SummedAreaTable::from_color_image(color_image);

    let mut result = BinaryImage::new_w_h(color_image.width, color_image.height);

    let half_patch_size = patch_size >> 1;
    for y in 0..result.height {
        for x in 0..result.width {
            let top_left = PointI32::new(std::cmp::max(0, (x - half_patch_size) as i32), std::cmp::max(0, (y - half_patch_size) as i32));
            let bot_right = PointI32::new(std::cmp::min(result.width as i32 - 1, (x + half_patch_size) as i32), std::cmp::min(result.height as i32 - 1, (y + half_patch_size) as i32));
            let threshold = std::cmp::max(0, sat.get_region_mean_top_left_bot_right(top_left, bot_right) as i32 - offset) as u8;

            let c = color_image.get_pixel(x, y);
            let c_mean = ((c.r as u32 + c.g as u32 + c.b as u32) / 3) as u8;
            result.set_pixel(x, y, c_mean < threshold);
        }
    }

    result
}

pub(crate) fn is_black_rgb(color: &Color) -> bool {
    let r = color.r as u32;
    let g = color.g as u32;
    let b = color.b as u32;

    r + g + b < 3*128
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
