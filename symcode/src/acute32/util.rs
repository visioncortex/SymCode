use visioncortex::{BinaryImage, Color, ColorImage, PointF64, PointI32, SampleStatBuilder};

pub(crate) fn binarize_image_util(color_image: &ColorImage) -> BinaryImage {
    let threshold = threshold_for(color_image);
    color_image.to_binary_image(move |c| {
        let r = c.r as u32;
        let g = c.g as u32;
        let b = c.b as u32;
    
        r + g + b < 3*threshold
    })
}

fn threshold_for(image: &ColorImage) -> u32 {
    let mut stat = SampleStatBuilder::new();
    for y in (0..image.height).step_by(8) {
        for x in 0..image.width {
            let c = image.get_pixel(x, y);
            stat.add((c.r as u32 + c.g as u32 + c.b as u32) as i32);
        }
    }
    stat.build();
    (stat.percentile(10) + stat.percentile(90)) as u32 / 6
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
