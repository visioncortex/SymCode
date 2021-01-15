use visioncortex::{BinaryImage, BoundingRect, Color, ColorHsv, ColorImage, PointF64};
use wasm_bindgen::{Clamped, JsValue};
use web_sys::{ImageData};

use crate::canvas::Canvas;

pub(crate) fn binarize_image(color_image: &ColorImage) -> BinaryImage {
    color_image.to_binary_image(|c| is_black_rgb(&c))
}

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

pub(crate) fn is_black_rgb(color: &Color) -> bool {
    let r = color.r as u32;
    let g = color.g as u32;
    let b = color.b as u32;

    r*r + g*g + b*b < 3*128*128
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

pub(crate) fn render_binary_image_to_canvas(image: &BinaryImage, canvas: &Canvas) -> Result<(), JsValue> {
    let image = &image.to_color_image();
    render_color_image_to_canvas(image, canvas)
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