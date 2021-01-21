use visioncortex::{BinaryImage, PointI32, Shape};
use wasm_bindgen::prelude::*;

use crate::{canvas::Canvas, util::console_log_util};

use super::{AlphabetReader, AlphabetReaderParams, FinderCandidate, GlyphLibrary, Recognizer, RecognizerInput, SymcodeConfig, glyph, implementation::transformer::{TransformFitter, TransformFitterInput}, is_black_hsv, pipeline::ScanningProcessor, render_binary_image_to_canvas};

#[wasm_bindgen]
#[derive(Default)]
pub struct SymcodeScanner {
    glyph_library: GlyphLibrary,
    config: SymcodeConfig,
}

#[wasm_bindgen]
impl SymcodeScanner {
    pub fn from_config(config: SymcodeConfig) -> Self {
        Self {
            glyph_library: GlyphLibrary::default(),
            config,
        }
    }

    /// Takes the id of the canvas element storing the template image, and the usize representation of the glyph label
    pub fn load_template_from_canvas_id(&mut self, canvas_id: &str) {
        let canvas = &match Canvas::new_from_id(canvas_id) {
            Some(c) => c,
            None => panic!("Canvas with id ".to_owned() + canvas_id + " is not found!"),
        };
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black_hsv(&c.to_hsv()));
        self.glyph_library.add_template(image, self.config.stat_tolerance);
    }

    /// Takes the id of the canvas element storing the alphabet.
    pub fn load_alphabet_from_canvas_id(&mut self, canvas_id: &str, params: AlphabetReaderParams) {
        let canvas = &match Canvas::new_from_id(canvas_id) {
            Some(c) => c,
            None => panic!("Canvas with id ".to_owned() + canvas_id + " is not found!"),
        };
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black_hsv(&c.to_hsv()));
        if let Some(e) = AlphabetReader::read_alphabet_to_library(&mut self.glyph_library, image, params, self.config.stat_tolerance).err() {
            console_log_util(e);
        }
    }

    pub fn scan(&self) -> String {
        if self.glyph_library.is_empty() {
            return "No templates loaded into the SymcodeScanner instance yet!".into();
        }

        let symcode_config = &self.config;

        // Stage 0: Prepare the raw input
        let raw_frame = if let Some(canvas) = &symcode_config.canvas {
            canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
        } else {
            return "Cannot read input image from canvas.".into();
        };
        
        // Stage 1: Locate finder candidates
        let finder_positions = match FinderCandidate::process(
            &raw_frame,
            symcode_config
        ) {
            Ok(finder_positions) => finder_positions,
            Err(e) => {
                return "Failed at Stage 1: ".to_owned() + e;
            }
        };
        
        // Stage 2: Fit a perspective transform from the image space to the object space
        let image_to_object = match TransformFitter::process(
            TransformFitterInput {
                finder_positions_image: finder_positions,
                raw_image_width: raw_frame.width,
                raw_image_height: raw_frame.height,
            },
            symcode_config
        ) {
            Ok(image_to_object) => image_to_object,
            Err(e) => {
                return "Failed at Stage 2: ".to_owned() + e;
            }
        };

        // Stage 3: Recognize the glyphs
        match Recognizer::process(
            RecognizerInput {
                raw_frame,
                image_to_object,
                glyph_library: &self.glyph_library,
            },
            symcode_config
        ) {
            Ok(glyph_code) => {
                console_log_util(&format!("{:?}", glyph_code));
                
                "Success".into()
            },
            Err(e) => {
                "Failed at Stage 3: ".to_owned() + e
            }
        }
    }

    pub fn generate_symcode_to_canvas(&self) {
        let canvas = self.config.canvas.as_ref().unwrap();
        let symcode = self.generate_symcode();

        if render_binary_image_to_canvas(&symcode, canvas).is_err() {
            console_log_util("Cannot render generated symcode to canvas.");
        }
    }

    fn generate_symcode(&self) -> BinaryImage {
        let mut symcode = BinaryImage::new_w_h(self.config.code_width, self.config.code_height);

        // Put in the finders
        let finder_image = Shape::circle(self.config.symbol_width, self.config.symbol_height).image;
        self.config.finder_positions.iter().for_each(|finder_center| {
            let top_left = finder_center.to_point_i32() - PointI32::new((self.config.symbol_width >> 1) as i32, (self.config.symbol_height >> 1) as i32);
            symcode.paste_from(&finder_image, top_left);
        });

        // Put in the glyphs
        self.config.glyph_anchors.iter().for_each(|glyph_top_left| {
            let glyph_index: usize = rand::random();
            let glyph_index = glyph_index % (self.glyph_library.len() + 1);

            // if glyph_index == glyph_library.len(), this will return None
            if let Some(glyph_image) = self.glyph_library.get_glyph_image_at(glyph_index) {
                symcode.paste_from(glyph_image, glyph_top_left.to_point_i32());
            }
        });

        symcode
    }
}