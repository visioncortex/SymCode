use std::borrow::BorrowMut;

use rand::{Rng, RngCore, SeedableRng, rngs::StdRng};
use visioncortex::{BinaryImage, PointI32, Shape};
use wasm_bindgen::prelude::*;

use crate::{canvas::Canvas, util::console_log_util};

use super::{AlphabetReader, AlphabetReaderParams, FinderCandidate, GlyphLibrary, Recognizer, RecognizerInput, SymcodeConfig, SymcodeDecoder, implementation::transformer::{TransformFitter, TransformFitterInput}, is_black_hsv, render_binary_image_to_canvas};

#[wasm_bindgen]
pub struct SymcodeScanner {
    glyph_library: GlyphLibrary,
    config: SymcodeConfig,
    rng: StdRng,
}

#[wasm_bindgen]
impl SymcodeScanner {
    pub fn from_config(config: SymcodeConfig, seed: u64) -> Self {
        Self {
            glyph_library: GlyphLibrary::default(),
            config,
            rng: StdRng::seed_from_u64(seed),
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
        self.glyph_library.add_template(image, &self.config);
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
        if let Some(e) = AlphabetReader::read_alphabet_to_library(&mut self.glyph_library, image, params, &self.config).err() {
            console_log_util(e);
        }
    }

    pub fn scan_from_canvas_id(&self, canvas_id: &str) -> Result<String, JsValue> {
        if self.glyph_library.is_empty() {
            return Err("No templates loaded into the SymcodeScanner instance yet!".into());
        }

        let symcode_config = &self.config;

        // Stage 0: Prepare the raw input
        let raw_frame = if let Some(canvas) = &Canvas::new_from_id(canvas_id) {
            canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
        } else {
            return Err("Cannot read input image from canvas.".into());
        };
        
        // Stage 1: Locate finder candidates
        let finder_positions = match FinderCandidate::process(
            &raw_frame,
            symcode_config
        ) {
            Ok(finder_positions) => finder_positions,
            Err(e) => {
                return Err(("Failed at Stage 1: ".to_owned() + e).into());
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
                return Err(("Failed at Stage 2: ".to_owned() + e).into());
            }
        };

        // Stage 3: Recognize the glyphs
        let symcode_instance = match Recognizer::process(
            RecognizerInput {
                raw_frame,
                image_to_object,
                glyph_library: &self.glyph_library,
            },
            symcode_config
        ) {
            Ok(symcode_instance) => symcode_instance,
            Err(e) => {
                return Err(("Failed at Stage 3: ".to_owned() + e).into());
            }
        };
        let debug_code_string = format!("{:?}", symcode_instance);

        // Stage 4: Decode the Symcode
        match SymcodeDecoder::process(
            symcode_instance
        ) {
            Ok(decoded_symcode) => {
                Ok(format!("{}\n{:?}", debug_code_string, decoded_symcode))
            },
            Err(e) => {
                Err(("Failed at Stage 4: ".to_owned() + e).into())
            }
        }
    }

    pub fn generate_symcode_to_canvas(&mut self, canvas_id: &str) -> Result<String, JsValue> {
        let canvas = if let Some(canvas) = Canvas::new_from_id(canvas_id) {
            canvas
        } else {
            return Err("Code generation: Canvas does not exist.".into());
        };
        let (symcode, ground_truth_code) = self.borrow_mut().generate_symcode();

        if render_binary_image_to_canvas(&symcode, &canvas).is_err() {
            return Err("Cannot render generated symcode to canvas.".into());
        }

        Ok(ground_truth_code)
    }

    fn generate_symcode(&mut self) -> (BinaryImage, String) {
        let mut symcode = BinaryImage::new_w_h(self.config.code_width, self.config.code_height);

        // Put in the finders
        let finder_image = Shape::circle(self.config.symbol_width, self.config.symbol_height).image;
        self.config.finder_positions.iter().for_each(|finder_center| {
            let top_left = finder_center.to_point_i32() - PointI32::new((self.config.symbol_width >> 1) as i32, (self.config.symbol_height >> 1) as i32);
            symcode.paste_from(&finder_image, top_left);
        });

        // Put in the glyphs
        let mut ground_truth_code = vec![];
        self.config.glyph_anchors.clone().iter().for_each(|glyph_top_left| {
            let glyph_index: usize = self.rng.next_u64() as usize;
            let glyph_index = glyph_index % (self.glyph_library.len() + 1);

            // if glyph_index == glyph_library.len(), this will return None
            if let Some(glyph) = self.glyph_library.get_glyph_at(glyph_index) {
                ground_truth_code.push(Some(glyph.label));
                symcode.paste_from(&glyph.image, glyph_top_left.to_point_i32());
            } else {
                ground_truth_code.push(None);
            }
        });

        let ground_truth_code_string = format!("{:?}", ground_truth_code);
        //console_log_util(&format!("Generated glyphs: {}", ground_truth_code_string));

        let ground_truth_code = SymcodeDecoder::process(ground_truth_code).unwrap();

        (symcode, format!("{}\n{:?}", ground_truth_code_string, ground_truth_code))
    }
}