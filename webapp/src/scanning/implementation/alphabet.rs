use visioncortex::{BinaryImage, BoundingRect, PointI32};
use wasm_bindgen::prelude::*;

use crate::scanning::{SymcodeConfig, valid_pointi32_on_image};

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

#[wasm_bindgen]
impl AlphabetReaderParams {
    pub fn from_json_string(json_string: &str) -> Self {
        let json: serde_json::Value = serde_json::from_str(json_string).unwrap();

        Self {
            top_left: PointI32::new(json["top_left"]["x"].as_i64().unwrap() as i32, json["top_left"]["y"].as_i64().unwrap() as i32),
            symbol_width: json["symbol_width"].as_i64().unwrap() as usize,
            symbol_height: json["symbol_height"].as_i64().unwrap() as usize,
            offset_x: json["offset_x"].as_i64().unwrap() as usize,
            offset_y: json["offset_y"].as_i64().unwrap() as usize,
            num_columns: json["num_columns"].as_i64().unwrap() as usize,
            num_rows: json["num_rows"].as_i64().unwrap() as usize,
        }
    }
}

impl<'a> AlphabetReader {
    pub fn read_alphabet_to_library(library: &'a mut GlyphLibrary, image: BinaryImage, params: AlphabetReaderParams, symcode_config: &'a SymcodeConfig) -> Result<(), &'a str> {
        for i in 0..params.num_rows {
            for j in 0..params.num_columns {
                let offset = PointI32::new((j * params.offset_x) as i32, (i * params.offset_y) as i32);
                let top_left = params.top_left + offset;
                let rect = BoundingRect::new_x_y_w_h(top_left.x, top_left.y, params.symbol_width as i32, params.symbol_height as i32);

                if !valid_pointi32_on_image(top_left, image.width, image.height) || !valid_pointi32_on_image(PointI32::new(rect.right, rect.bottom), image.width, image.height) {
                    return Err("AlphabetReader error: trying to crop out of image bound.");
                }

                let glyph_image = image.crop_with_rect(rect);
                library.add_template(glyph_image, symcode_config);
            }
        }
        //crate::util::console_log_util(&library.print_label_and_trace());
        Ok(())
    }
}