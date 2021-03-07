use wasm_bindgen::{Clamped, JsValue};
use web_sys::ImageData;
use visioncortex::{BoundingRect, Color, BinaryImage, ColorImage};
use crate::interfaces::Debugger as DebuggerInterface;
use crate::canvas::Canvas;

pub struct Debugger {
    pub(crate) debug_canvas: Canvas,
}

impl DebuggerInterface for Debugger {
	fn render_color_image_to_canvas(&self, image: &ColorImage) -> Result<(), &'static str> {
		render_color_image_to_canvas(&self.debug_canvas, image)
	}

	fn render_bounding_rect_to_canvas_with_color(&self, rect: &BoundingRect, color: Color) {
	    let ctx = self.debug_canvas.get_rendering_context_2d();
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
}

pub fn render_color_image_to_canvas(canvas: &Canvas, image: &ColorImage) -> Result<(), &'static str> {
    let mut data = image.pixels.clone();
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), image.width as u32, image.height as u32).unwrap();
    let ctx = canvas.get_rendering_context_2d();
    canvas.set_width(image.width);
    canvas.set_height(image.height);
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    if ctx.put_image_data(&data, 0.0, 0.0).is_err() {
        Err("failed to put_image_data")
    } else {
        Ok(())
    }
}

pub fn render_binary_image_to_canvas(canvas: &Canvas, image: &BinaryImage) -> Result<(), &'static str> {
    let image = &image.to_color_image();
    render_color_image_to_canvas(canvas, image)
}