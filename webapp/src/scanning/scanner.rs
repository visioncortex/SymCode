use wasm_bindgen::prelude::*;

use crate::{canvas::Canvas, util::console_log_util};

use super::{AlphabetReader, AlphabetReaderParams, FinderCandidate, GlyphLibrary, Recognizer, RecognizerInput, SymcodeConfig, implementation::transformer::{TransformFitter, TransformFitterInput}, is_black_hsv, pipeline::ScanningProcessor};

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
        AlphabetReader::read_alphabet_to_library(&mut self.glyph_library, image, params, self.config.stat_tolerance);
    }

    pub fn scan(&self) -> JsValue {
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
                return ("Failed at Stage 1: ".to_owned() + e).into();
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
                return ("Failed at Stage 2: ".to_owned() + e).into();
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
                ("Failed at Stage 3: ".to_owned() + e).into()
            }
        }
    }
}