use visioncortex::{BinaryImage, BoundingRect, PointI32};

use super::GlyphLibrary;

pub struct AlphabetReader {}

pub struct AlphabetReaderParams {
    // top-left point of the top-left glyph
    pub top_left: PointI32,
    pub glyph_width: usize,
    pub glyph_height: usize,
    pub offset_x: usize,
    pub offset_y: usize,
    pub num_columns: usize,
    pub num_rows: usize,
}

impl AlphabetReader {
    pub fn read_alphabet(image: BinaryImage, params: AlphabetReaderParams) -> GlyphLibrary {
        let mut library = GlyphLibrary::default();

        for i in 0..params.num_rows {
            for j in 0..params.num_columns {
                let offset = PointI32::new((j * params.offset_x) as i32, (i * params.offset_y) as i32);
                let top_left = params.top_left + offset;
                let rect = BoundingRect::new_x_y_w_h(top_left.x, top_left.y, params.glyph_width as i32, params.glyph_height as i32);
                let label = i*params.num_columns + j;

                library.add_template(image.crop_with_rect(rect), label);
            }
        }

        library
    }
}