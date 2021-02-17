use visioncortex::{BinaryImage, Sampler};

use crate::{scanning::{SymcodeConfig, image_diff_area, Trace}};

use super::{Glyph, GlyphLabel, GlyphTrace};

#[derive(Debug)]
pub struct GlyphLibrary {
    templates: Vec<Glyph>,
}

impl Default for GlyphLibrary {
    fn default() -> Self {
        Self { templates: vec![] }
    }
}

impl GlyphLibrary {
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn get_glyph_at(&self, i: usize) -> Option<&Glyph> {
        if i >= self.templates.len() {
            None
        } else {
            Some(&self.templates[i])
        }
    }

    pub fn get_glyph_with_label(&self, label: GlyphLabel) -> Option<&Glyph> {
        for glyph in self.templates.iter() {
            if glyph.label == label {
                return Some(glyph);
            }
        }
        None
    }

    pub fn print_label_and_trace(&self) -> String {
        let list: Vec<String> = self.templates.iter().map(|glyph| {
            format!("{:?}: {:?}\n", glyph.label, glyph.encoding.bits)
        }).collect();
        list.join("")
    }

    /// Takes the binary image of the template and the usize representation of the label
    pub fn add_template(&mut self, image: BinaryImage, symcode_config: &SymcodeConfig) {
        let image = Sampler::resample_image(&image, symcode_config.symbol_width, symcode_config.symbol_height);
        let label = GlyphLabel::from_usize_representation(self.templates.len() + 1);
        //console_log_util(&format!("{:?}\n{}", label, image.to_string()));
        self.templates.push(Glyph::from_image_label(image, label, symcode_config.stat_tolerance));
    }

    pub fn find_most_similar_glyph(&self, image: BinaryImage, symcode_config: &SymcodeConfig) -> GlyphLabel {
        let image = &Sampler::resample_image(&image, symcode_config.symbol_width, symcode_config.symbol_height);
        let input_encoding = &GlyphTrace::from_image(image, symcode_config.stat_tolerance);
        //console_log_util(&format!("{:?}", input_encoding));

        self.templates.iter()
            .fold( (std::u64::MAX, GlyphLabel::Invalid),
                |(min_error, min_label), template| {
                    if template.encoding.diff(input_encoding) > symcode_config.max_encoding_difference {
                        return (min_error, min_label);
                    }
                    let error = image_diff_area(&template.image, image);
                    if error < min_error {
                        (error, template.label)
                    } else {
                        (min_error, min_label)
                    }
                }
            ).1
    }
}