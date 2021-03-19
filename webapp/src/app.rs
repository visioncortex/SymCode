use bit_vec::BitVec;
use rand::{RngCore, SeedableRng, rngs::StdRng};
use visioncortex::{BinaryImage, ColorImage, PointI32};
use wasm_bindgen::prelude::*;

use symcode::acute32::{Acute32, Acute32SymcodeConfig, AlphabetReader, AlphabetReaderParams, GlyphLabel};
use symcode::interfaces::{Decoder, Finder, FinderElement, Fitter, Reader, Encoder, SymcodeScanner, SymcodeGenerator};
use symcode::math::{into_bitvec, num_bits_to_store, num_significant_bits};
use crate::{canvas::Canvas, util::console_log_util};
use crate::debugger::{Debugger, render_binary_image_to_canvas};
use super::helper::is_black_hsv;

#[wasm_bindgen]
pub struct Acute32SymcodeMain {
    config: Acute32SymcodeConfig,
    rng: StdRng,
}

impl Default for Acute32SymcodeMain {
    fn default() -> Self {
        Self::new()
    }
}

impl Acute32SymcodeMain {
    pub fn from_config(config: Acute32SymcodeConfig, seed: u64) -> Self {
        Self {
            config,
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

#[wasm_bindgen]
impl Acute32SymcodeMain {
    pub fn new() -> Self {
        let mut config = Acute32SymcodeConfig::default();
        if let Some(debug_canvas) = Canvas::new_from_id("debug") {
            config.debugger = Box::new(Debugger{ debug_canvas });
        }
        Self::from_config(config, 125)
    }

    pub fn seed_rng(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
    } 

    /// Takes the id of the canvas element storing the alphabet.
    pub fn load_alphabet_from_canvas_id(&mut self, canvas_id: &str) {
        let params = AlphabetReaderParams::default();
        let canvas = &match Canvas::new_from_id(canvas_id) {
            Some(c) => c,
            None => panic!("Canvas with id ".to_owned() + canvas_id + " is not found!"),
        };
        let image = canvas
            .get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
            .to_binary_image(|c| is_black_hsv(&c.to_hsv()));
        match AlphabetReader::read_alphabet_to_library(image, params, &self.config) {
            Ok(library) => self.config.symbol_library = Box::new(library),
            Err(e) => console_log_util(e),
        }
    }

    pub fn scan_from_canvas_id(&self, canvas_id: &str) -> Result<String, JsValue> {
        if self.config.symbol_library.is_empty() {
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
        Ok(format!("{:?}", decoded_bit_string))
    }

    // Payload should be encodable in 20 bits max
    pub fn generate_symcode_to_canvas(&self, canvas_id: &str, payload: usize) -> Result<String, JsValue> {
        if num_significant_bits(payload) > 20 {
            return Err("Payload has too many bits!".into());
        }

        let canvas = if let Some(canvas) = Canvas::new_from_id(canvas_id) {
            canvas
        } else {
            return Err("Code generation: Canvas does not exist.".into());
        };

        let (symcode, ground_truth_code) = self.generate_symcode_with_payload(payload)?;

        if render_binary_image_to_canvas(&canvas, &symcode).is_err() {
            return Err("Cannot render generated symcode to canvas.".into());
        }

        Ok(ground_truth_code)
    }

    pub fn generate_random_symcode_to_canvas(&mut self, canvas_id: &str) -> Result<String, JsValue> {
        let canvas = if let Some(canvas) = Canvas::new_from_id(canvas_id) {
            canvas
        } else {
            return Err("Code generation: Canvas does not exist.".into());
        };
        let (symcode, ground_truth_code) = self.generate_symcode_random()?;

        if render_binary_image_to_canvas(&canvas, &symcode).is_err() {
            return Err("Cannot render generated symcode to canvas.".into());
        }

        Ok(ground_truth_code)
    }

    fn generate_symcode_with_payload(&self, payload: usize) -> Result<(BinaryImage, String), &str> {
        let payload = into_bitvec(payload, 20);
        let payload_bit_string = format!("{:?}", payload);

        let num_symbols = self.config.num_glyphs_in_code();

        let acute32 = Acute32::new(&self.config);
        let symcode_representation = acute32.get_encoder().encode(payload, num_symbols)?;
        let symcode_string = format!("{:?}", symcode_representation);

        let code_image = self.generate(symcode_representation);

        let msg = format!("{}\n{}", symcode_string, payload_bit_string);

        //console_log_util(&msg);

        Ok((code_image, msg)) 
    }

    /// Randomly generate a 20-bit bit string, calculate CRC5 checksum (which is 5 bits)
    /// Then encode the 25-bit bit string into a symcode and generate the code image
    fn generate_symcode_random(&mut self) -> Result<(BinaryImage, String), &str> {
        let symbol_num_bits = num_bits_to_store(GlyphLabel::num_variants());
        let num_symbols = self.config.num_glyphs_in_code();

        // Dummy data
        let payload = BitVec::from_fn(
            symbol_num_bits*num_symbols - 5, // Reserve 5 bits for CRC5 checksum
            |_| { self.rng.next_u32() < (std::u32::MAX >> 1) }
        );
        let payload_bit_string = format!("{:?}", payload);

        let acute32 = Acute32::new(&self.config);
        let symcode_representation = acute32.get_encoder().encode(payload, num_symbols)?;
        let symcode_string = format!("{:?}", symcode_representation);

        let code_image = self.generate(symcode_representation);

        let msg = format!("{}\n{}", symcode_string, payload_bit_string);

        //console_log_util(&msg);

        Ok((code_image, msg))
    }
}

impl SymcodeScanner for Acute32SymcodeMain {
    type SymcodeRepresentation = Vec<GlyphLabel>;

    type Err = JsValue;

    fn scan(&self, image: ColorImage) -> Result<Self::SymcodeRepresentation, Self::Err> {
        let acute32 = Acute32::new(&self.config);

        // Stage 1: Locate finder candidates
        let finder_positions = match acute32.get_finder().find(
            &image
        ) {
            Ok(finder_positions) => finder_positions,
            Err(e) => {
                return Err(("Failed at Stage 1: ".to_owned() + e).into());
            }
        };

        // Stage 2: Fit a perspective transform from the image space to the object space
        let image_to_object = match acute32.get_fitter().fit(
            finder_positions,
            image.width,
            image.height
        ) {
            Ok(image_to_object) => image_to_object,
            Err(e) => {
                return Err(("Failed at Stage 2: ".to_owned() + e).into());
            }
        };

        // Stage 3: Recognize the glyphs
        let symcode_instance = match acute32.get_reader().read(
            image,
            image_to_object
        ) {
            Ok(symcode_instance) => symcode_instance,
            Err(e) => {
                return Err(("Failed at Stage 3: ".to_owned() + e).into());
            }
        };

        Ok(symcode_instance)
    }

    fn decode(&self, symcode: Self::SymcodeRepresentation) -> Result<bit_vec::BitVec, Self::Err> {
        let acute32 = Acute32::new(&self.config);

        // Stage 4: Decode the Symcode
        match acute32.get_decoder().decode(
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

impl SymcodeGenerator for Acute32SymcodeMain {
    type SymcodeRepresentation = Vec<GlyphLabel>;

    fn generate(&self, symcode: Self::SymcodeRepresentation) -> BinaryImage {
        let mut symcode_image = BinaryImage::new_w_h(self.config.code_width, self.config.code_height);

        // Put in the finders
        let finder_image = self.config.finder.to_image(self.config.symbol_width, self.config.symbol_height);
        self.config.finder_positions.iter().for_each(|finder_center| {
            let top_left = finder_center.to_point_i32() - PointI32::new((self.config.symbol_width >> 1) as i32, (self.config.symbol_height >> 1) as i32);
            symcode_image.paste_from(&finder_image, top_left);
        });

        // Put in the glyphs
        symcode.iter().enumerate().for_each(|(i, &glyph_label)| {
            if glyph_label != GlyphLabel::Invalid {
                let glyph_top_left = self.config.glyph_anchors[i];
                if let Some(glyph) = self.config.symbol_library.get_glyph_with_label(glyph_label) {
                    symcode_image.paste_from(&glyph.image, glyph_top_left.to_point_i32());
                }
            }
        });

        symcode_image
    }
}