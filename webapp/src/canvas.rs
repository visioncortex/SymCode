use wasm_bindgen::{JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use visioncortex::{ColorImage};

use super::common::document;

pub struct Canvas {
    html_canvas: HtmlCanvasElement,
    cctx: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new_from_id(canvas_id: &str) -> Option<Canvas> {
        let html_canvas = document().get_element_by_id(canvas_id)?;
        let html_canvas: HtmlCanvasElement = html_canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let cctx = html_canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Some(
            Canvas {
                html_canvas,
                cctx,
            }
        )
    }

    pub fn get_rendering_context_2d(&self) -> &CanvasRenderingContext2d {
        &self.cctx
    }

    pub fn width(&self) -> usize {
        self.html_canvas.width() as usize
    }

    pub fn set_width(&self, value: usize) {
        self.html_canvas.set_width(value as u32);
    }

    pub fn height(&self) -> usize {
        self.html_canvas.height() as usize
    }

    pub fn set_height(&self, value: usize) {
        self.html_canvas.set_height(value as u32);
    }

    pub fn get_image_data(&self, x: u32, y: u32, width: u32, height: u32) -> Vec<u8> {
        let image = self
            .cctx
            .get_image_data(x as f64, y as f64, width as f64, height as f64)
            .unwrap();
        image.data().to_vec()
    }

    pub fn get_image_data_as_color_image(&self, x: u32, y: u32, width: u32, height: u32) -> ColorImage {
        ColorImage {
            pixels: self.get_image_data(x, y, width, height),
            width: width as usize,
            height: height as usize,
        }
    }
}