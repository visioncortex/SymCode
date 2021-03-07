use visioncortex::{BinaryImage, BoundingRect, PointI32};
use crate::acute32::{Acute32SymcodeConfig, valid_pointi32_on_image};
use super::Acute32Library;

pub struct AlphabetReader;

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
            top_left: PointI32::new(100, 100),
            symbol_width: 155,
            symbol_height: 155,
            offset_x: 155.0*1.5,
            offset_y: 155.0*1.5,
            num_columns: 4,
            num_rows: 8,
        }
    }
}

impl AlphabetReaderParams {
    pub fn new() -> Self {
        Self::default()
    }

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

impl AlphabetReader {
    pub fn read_alphabet_to_library(image: BinaryImage, params: AlphabetReaderParams, symcode_config: &Acute32SymcodeConfig) -> Result<Acute32Library, &'static str> {
        let mut library = Acute32Library::default();
        symcode_config.debugger.render_binary_image_to_canvas(&image)?;
        for i in 0..params.num_rows {
            for j in 0..params.num_columns {
                let offset = PointI32::new((j as f64 * params.offset_x) as i32, (i as f64 * params.offset_y) as i32);
                let top_left = params.top_left + offset;
                let rect = BoundingRect::new_x_y_w_h(top_left.x, top_left.y, params.symbol_width as i32, params.symbol_height as i32);
                symcode_config.debugger.render_bounding_rect_to_canvas(&rect);
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