use visioncortex::{BoundingRect, ColorImage, color_clusters::Cluster};
use wasm_bindgen::{Clamped, JsValue};
use web_sys::{ImageData, console};

use crate::canvas::Canvas;

extern crate cfg_if;

cfg_if::cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

/*
/// First attempt: When locating Finder patterns, make the shape square before checking if it is a circle to deal with distortion
pub(crate) fn make_shape_square(original: &Shape) -> Shape {
    if original.image.width == original.image.height {
        original.clone()
    } else {
        let max_side = std::cmp::max(original.image.width, original.image.height);
        Shape {
            image: Sampler::resample_image(&original.image, max_side, max_side)
        }
    }
}
*/

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