use visioncortex::{BinaryImage, BoundingRect, PointI32};
use wasm_bindgen::prelude::*;

use crate::acute32::{Acute32SymcodeConfig, valid_pointi32_on_image};

use super::Acute32Library;

pub struct AlphabetReader {}

#[wasm_bindgen]
pub struct AlphabetReaderParams {
    // top-left point of the top-left glyph
    pub(crate) top_left: PointI32,
    pub(crate) symbol_width: usize,
    pub(crate) symbol_height: usize,
    pub(crate) offset_x: f64,
    pub(crate) offset_y: f64,
    pub(crate) num_columns: usize,
    pub(crate) num_rows: usize,
}

impl Default for AlphabetReaderParams {
    fn default() -> Self {
        Self {
            top_left: PointI32::new(0, 0),
            symbol_width:80,
            symbol_height: 80,
            offset_x: 115.0,
            offset_y: 115.0,
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

    pub fn offset(mut self, x: f64, y: f64) -> Self {
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
            offset_x: json["offset_x"].as_f64().unwrap(),
            offset_y: json["offset_y"].as_f64().unwrap(),
            num_columns: json["num_columns"].as_i64().unwrap() as usize,
            num_rows: json["num_rows"].as_i64().unwrap() as usize,
        }
    }
}

impl AlphabetReader {
    pub fn read_alphabet_to_library(image: BinaryImage, params: AlphabetReaderParams, symcode_config: &Acute32SymcodeConfig) -> Result<Acute32Library, &'static str> {
        let mut library = Acute32Library::default();
        crate::acute32::util::render_binary_image_to_canvas(&image, symcode_config.debug_canvas.as_ref().unwrap())?;
        for i in 0..params.num_rows {
            for j in 0..params.num_columns {
                let offset = PointI32::new((j as f64 * params.offset_x) as i32, (i as f64 * params.offset_y) as i32);
                let top_left = params.top_left + offset;
                let rect = BoundingRect::new_x_y_w_h(top_left.x, top_left.y, params.symbol_width as i32, params.symbol_height as i32);
                crate::acute32::util::render_bounding_rect_to_canvas(&rect, symcode_config.debug_canvas.as_ref().unwrap());
                if !valid_pointi32_on_image(top_left, image.width, image.height) || !valid_pointi32_on_image(PointI32::new(rect.right, rect.bottom), image.width, image.height) {
                    return Err("AlphabetReader error: trying to crop out of image bound.");
                }

                let glyph_image = image.crop_with_rect(rect);
                library.add_template(glyph_image, symcode_config);
            }
        }
        //crate::util::console_log_util(&library.get_labels_grouped_by_trace());
        Ok(library)
    }
}