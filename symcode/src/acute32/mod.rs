mod alphabet;
mod decoder;
mod encoder;
mod fitter;
mod finder;
mod label;
mod library;
mod reader;
mod symbol;
mod symcode_config;
mod trace;
mod util;

pub use alphabet::*;
pub use decoder::*;
pub use encoder::*;
pub use finder::*;
pub use fitter::*;
pub use label::*;
pub use library::*;
pub use reader::*;
pub use symbol::*;
pub use symcode_config::*;
pub use trace::*;
use util::*;

pub struct Acute32<'a> {
    config: &'a Acute32SymcodeConfig,
}

impl<'a> Acute32<'a> {

    pub fn new(config: &'a Acute32SymcodeConfig) -> Acute32<'a> {
        Self { config }
    }

	pub fn get_finder(&self) -> Acute32FinderCandidate {
		Acute32FinderCandidate::new(self.config)
	}

	pub fn get_fitter(&self) -> Acute32TransformFitter {
		Acute32TransformFitter::new(self.config)
	}

	pub fn get_reader(&self) -> Acute32Recognizer {
		Acute32Recognizer::new(self.config)
	}

	pub fn get_decoder(&self) -> Acute32Decoder {
		Acute32Decoder::new(self.config)
	}

	pub fn get_encoder(&self) -> Acute32Encoder {
		Acute32Encoder::new(self.config)
	}
}