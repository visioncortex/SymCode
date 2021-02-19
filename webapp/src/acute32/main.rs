use std::borrow::BorrowMut;

use rand::{RngCore, SeedableRng, rngs::StdRng};
use visioncortex::{BinaryImage, ColorImage, PointI32};
use wasm_bindgen::prelude::*;

use crate::{canvas::Canvas, interfaces::finder::Finder as FinderInterface, util::console_log_util};

use super::{AlphabetReader, AlphabetReaderParams, Acute32FinderCandidate, GlyphLabel, Acute32Recognizer, RecognizerInput, Acute32SymcodeConfig, Acute32Decoder, implementation::transformer::{Acute32TransformFitter, TransformFitterInput}, is_black_hsv, render_binary_image_to_canvas};

use crate::interfaces::scanner::SymcodeScanner as ScannerInterface;
use crate::interfaces::generator::SymcodeGenerator as GeneratorInterface;

#[wasm_bindgen]
pub struct Acute32SymcodeMain {
    config: Acute32SymcodeConfig,
    rng: StdRng,
}

#[wasm_bindgen]
impl Acute32SymcodeMain {
    pub fn from_config(config: Acute32SymcodeConfig, seed: u64) -> Self {
        Self {
            config,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn seed_rng(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
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
        match AlphabetReader::read_alphabet_to_library(image, params, &self.config) {
            Ok(library) => self.config.set_library(library),
            Err(e) => console_log_util(e),
        }
    }

    pub fn scan_from_canvas_id(&self, canvas_id: &str) -> Result<String, JsValue> {
        if self.config.library().is_empty() {
            return Err("No templates loaded into the SymcodeScanner instance yet!".into());
        }

        // Stage 0: Prepare the raw input
        let raw_frame = if let Some(canvas) = &Canvas::new_from_id(canvas_id) {
            canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
        } else {
            return Err("Cannot read input image from canvas.".into());
        };

        let symcode = self.scan(raw_frame)?;
        let debug_code_string = format!("{:?}", symcode);

        let decoded_bit_string = self.decode(symcode)?;
        Ok(format!("{}\n{:?}", debug_code_string, decoded_bit_string))
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
        let mut ground_truth_code = vec![];
        for _ in 0..self.config.num_glyphs_in_code() {
            let glyph_index: usize = self.rng.next_u64() as usize;
            let glyph_index = glyph_index % (self.config.library().len() + 1);
            
            // if glyph_index == glyph_library.len(), this will return None
            if let Some(glyph) = self.config.library().get_glyph_at(glyph_index) {
                ground_truth_code.push(glyph.label);
            } else {
                ground_truth_code.push(GlyphLabel::Empty);
            }
        }
        
        let ground_truth_code_as_string = format!("{:?}", ground_truth_code);
        //console_log_util(&format!("Generated glyphs: {}", ground_truth_code_string));

        let ground_truth_code_as_bits = Acute32Decoder::process(ground_truth_code.clone()).unwrap();

        let symcode_image = self.generate(ground_truth_code);

        (symcode_image, format!("{}\n{:?}", ground_truth_code_as_string, ground_truth_code_as_bits))
    }
}

impl ScannerInterface for Acute32SymcodeMain {
    type SymcodeRepresentation = Vec<GlyphLabel>;

    type Err = JsValue;

    fn scan(&self, image: ColorImage) -> Result<Self::SymcodeRepresentation, Self::Err> {
        let symcode_config = &self.config;
        // Stage 1: Locate finder candidates
        let finder_positions = match Acute32FinderCandidate::process(
            &image,
            symcode_config
        ) {
            Ok(finder_positions) => finder_positions,
            Err(e) => {
                return Err(("Failed at Stage 1: ".to_owned() + e).into());
            }
        };
        
        // Stage 2: Fit a perspective transform from the image space to the object space
        let image_to_object = match Acute32TransformFitter::process(
            TransformFitterInput {
                finder_positions_image: finder_positions,
                raw_image_width: image.width,
                raw_image_height: image.height,
            },
            symcode_config
        ) {
            Ok(image_to_object) => image_to_object,
            Err(e) => {
                return Err(("Failed at Stage 2: ".to_owned() + e).into());
            }
        };

        // Stage 3: Recognize the glyphs
        let symcode_instance = match Acute32Recognizer::process(
            RecognizerInput {
                raw_frame: image,
                image_to_object,
                glyph_library: self.config.library(),
            },
            symcode_config
        ) {
            Ok(symcode_instance) => symcode_instance,
            Err(e) => {
                return Err(("Failed at Stage 3: ".to_owned() + e).into());
            }
        };

        Ok(symcode_instance)
    }

    fn decode(&self, symcode: Self::SymcodeRepresentation) -> Result<bit_vec::BitVec, Self::Err> {
        // Stage 4: Decode the Symcode
        match Acute32Decoder::process(
            symcode
        ) {
            Ok(decoded_symcode) => {
                Ok(decoded_symcode)
            },
            Err(e) => {
                Err(("Failed at Stage 4: ".to_owned() + e).into())
            }
        }
    }
}

impl GeneratorInterface for Acute32SymcodeMain {
    type SymcodeRepresentation = Vec<GlyphLabel>;

    fn generate(&self, symcode: Self::SymcodeRepresentation) -> BinaryImage {
        let mut symcode_image = BinaryImage::new_w_h(self.config.code_width, self.config.code_height);

        // Put in the finders
        let finder_image = self.config.finder().to_image(self.config.symbol_width, self.config.symbol_height);
        self.config.finder_positions.iter().for_each(|finder_center| {
            let top_left = finder_center.to_point_i32() - PointI32::new((self.config.symbol_width >> 1) as i32, (self.config.symbol_height >> 1) as i32);
            symcode_image.paste_from(&finder_image, top_left);
        });

        // Put in the glyphs
        symcode.iter().enumerate().for_each(|(i, &glyph_label)| {
            if glyph_label != GlyphLabel::Empty {
                let glyph_top_left = self.config.glyph_anchors[i];
                if let Some(glyph) = self.config.library().get_glyph_with_label(glyph_label) {
                    symcode_image.paste_from(&glyph.image, glyph_top_left.to_point_i32());
                }
            }
        });

        symcode_image
    }
}