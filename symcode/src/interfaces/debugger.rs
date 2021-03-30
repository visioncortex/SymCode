use visioncortex::{BinaryImage, BoundingRect, Color, ColorImage, PointI32};

/// For use during development to help visualizing the pipeline stages
pub trait Debugger {

    fn render_color_image_to_canvas(&self, image: &ColorImage) -> Result<(), &'static str>;

    fn render_binary_image_to_canvas(&self, image: &BinaryImage) -> Result<(), &'static str> {
        let image = &image.to_color_image();
        self.render_color_image_to_canvas(image)
    }

    fn render_bounding_rect_to_canvas_with_color(&self, rect: &BoundingRect, color: Color);

    fn render_point_i32_to_canvas(&self, point: PointI32) {
        self.render_point_i32_to_canvas_with_size_color(point, 4, Color::new(255, 0, 0))
    }

    fn render_point_i32_to_canvas_with_size_color(&self, point: PointI32, size: usize, color: Color) {
        let rect = &BoundingRect::new_x_y_w_h(point.x - (size>>1) as i32, point.y - (size>>1) as i32, size as i32, size as i32);
        self.render_bounding_rect_to_canvas_with_color(rect, color)
    }

    fn render_bounding_rect_to_canvas(&self, rect: &BoundingRect) {
        self.render_bounding_rect_to_canvas_with_color(rect, Color::new(255, 0, 0));
    }

    fn log(&self, msg: &str);
}

#[derive(Default)]
pub struct DummyDebugger;

impl Debugger for DummyDebugger {

    fn render_color_image_to_canvas(&self, _image: &ColorImage) -> Result<(), &'static str> { Ok(()) }

    fn render_binary_image_to_canvas(&self, _image: &BinaryImage) -> Result<(), &'static str> { Ok(()) }

    fn render_bounding_rect_to_canvas_with_color(&self, _rect: &BoundingRect, _color: Color) {  }

    fn log(&self, msg: &str) { log::info!("{}", msg); }
}