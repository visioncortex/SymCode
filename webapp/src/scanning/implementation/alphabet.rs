use visioncortex::{BinaryImage, BoundingRect, PointI32};
use wasm_bindgen::prelude::*;

use super::GlyphLibrary;

pub struct AlphabetReader {}

#[wasm_bindgen]
pub struct AlphabetReaderParams {
    // top-left point of the top-left glyph
    pub(crate) top_left: PointI32,
    pub(crate) symbol_width: usize,
    pub(crate) symbol_height: usize,
    pub(crate) offset_x: usize,
    pub(crate) offset_y: usize,
    pub(crate) num_columns: usize,
    pub(crate) num_rows: usize,
}

impl Default for AlphabetReaderParams {
    fn default() -> Self {
        Self {
            top_left: PointI32::new(0, 0),
            symbol_width:80,
            symbol_height: 80,
            offset_x: 115,
            offset_y: 115,
            num_columns: 4,
            num_rows: 4,
        }
    }
}

#[wasm_bindgen]
impl AlphabetReaderParams {
    pub fn new() -> Self {
        Self::default()
    }

    // Can't use macros inside wasm_bindgen impls

    pub fn top_left(mut self, x: i32, y: i32) -> Self {
        self.top_left = PointI32::new(x, y);
        self
    }

    pub fn symbol_size(mut self, width: usize, height: usize) -> Self {
        self.symbol_width = width;
        self.symbol_height = height;
        self
    }

    pub fn offset(mut self, x: usize, y: usize) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    pub fn matrix_size(mut self, num_columns: usize, num_rows: usize) -> Self {
        self.num_columns = num_columns;
        self.num_rows = num_rows;
        self
    }
}

impl AlphabetReader {
    pub fn read_alphabet_to_library(library: &mut GlyphLibrary, image: BinaryImage, params: AlphabetReaderParams, stat_tolerance: f64) {
        for i in 0..params.num_rows {
            for j in 0..params.num_columns {
                let offset = PointI32::new((j * params.offset_x) as i32, (i * params.offset_y) as i32);
                let top_left = params.top_left + offset;
                let rect = BoundingRect::new_x_y_w_h(top_left.x, top_left.y, params.symbol_width as i32, params.symbol_height as i32);

                let glyph_image = image.crop_with_rect(rect);
                library.add_template(glyph_image, stat_tolerance);
            }
        }
        //console_log_util(&format!("{:?}", library));
    }
}