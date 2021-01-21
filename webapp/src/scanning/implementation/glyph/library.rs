use std::{fs, path::PathBuf};

use visioncortex::{BinaryImage, ColorImage, Sampler};

use crate::{scanning::{SymcodeConfig, image_diff_area, is_black_hsv}, util::console_log_util};

use super::{Glyph, GlyphLabel, ShapeEncoding};

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
    /// Takes the binary image of the template and the usize representation of the label
    pub fn add_template(&mut self, image: BinaryImage, stat_tolerance: f64) {
        let label = GlyphLabel::from_usize_representation(self.templates.len() + 1);
        //console_log_util(&format!("{:?}\n{}", label, image.to_string()));
        self.templates.push(Glyph::from_image_label(image, label, stat_tolerance));
    }

    pub fn find_most_similar_glyph(&self, image: BinaryImage, symcode_config: &SymcodeConfig) -> GlyphLabel {
        let image = &Sampler::resample_image(&image, symcode_config.symbol_width, symcode_config.symbol_height);
        let input_encoding = &ShapeEncoding::from_image(image, symcode_config.stat_tolerance);
        console_log_util(&format!("{:?}", input_encoding));

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

// For CMDAPP
impl GlyphLibrary {
    const DEFAULT_DIR: &'static str = "./";

    /// Loads the glyph templates in the specified directory as BinaryImage.
    ///
    /// Panics if path is not found or no jpg is found there.
    pub fn load_from_directory(path: &str, stat_tolerance: f64) -> Self {
        let mut path = String::from(path);

        if !path.ends_with('/') {
            path.push_str("/");
        }
        let dir = PathBuf::from(path.clone());
        console_log_util(&format!("{:?}", dir));
        if !dir.is_dir() {
            panic!("GlyphLibrary Error: Specified path ".to_owned() + &path + " is not a directory.");
        }

        if let Ok(entries) = fs::read_dir(dir) { // Read the directory
            Self {
                templates:
                    entries.into_iter()
                        .filter_map(|entry| { // Read each entry in the directory
                            if let Ok(file) = entry {
                                // Read the image in the entry
                                let file_name = file.file_name().into_string().unwrap();
                                println!("{}", &(path.to_owned() + &file_name));
                                Some(
                                    match read_image(&(path.clone() + &file_name)) {
                                        Ok(img) => (img.to_binary_image(|c| is_black_hsv(&c.to_hsv())), GlyphLabel::Empty), // Dummy label for category: figure it out later
                                        Err(e) => {
                                            console_log_util(&e);
                                            return None;
                                        },
                                    }
                                )
                            } else {
                                None
                            }
                        })
                        .map(|(image, label)| Glyph::from_image_label(image, label, stat_tolerance))
                        .collect()
            }
        } else {
            panic!("GlyphLibrary Error: Specified path ".to_owned() + &path + " cannot be read.");
        }
    }
}

// For CMDAPP
fn read_image(input_path: &str) -> Result<ColorImage, String> {
    let img = image::open(PathBuf::from(input_path));
    let img = match img {
        Ok(file) => file.to_rgba8(),
        Err(_) => return Err("No image file found at path ".to_owned() + input_path),
    };

    let (width, height) = (img.width() as usize, img.height() as usize);
    let img = ColorImage {pixels: img.as_raw().to_vec(), width, height};

    Ok(img)
}

// For CMDAPP
#[cfg(test)]
mod tests {
    use std::fs;

    use visioncortex::BinaryImage;

    use crate::scanning::is_black_hsv;

    use super::*;

    #[test]
    fn test_read_image() {
        let image = match read_image("dev/assets/test.jpg") {
            Ok(img) => img.to_binary_image(|c| is_black_hsv(&c.to_hsv())),
            Err(e) => panic!(e),
        };
        assert_eq!(image.to_string(), BinaryImage::from_string(&(
            "------------------------------\n".to_owned()+
            "------------------------------\n" +
            "------------------------------\n" +
            "------------------------------\n" +
            "------------------------------\n" +
            "----------------------*-------\n" +
            "---------------------**-------\n" +
            "---------------------*--------\n" +
            "---------------------*--------\n" +
            "--------------------*---------\n" +
            "--------------------*---------\n" +
            "--------------------*---------\n" +
            "-------------------*----------\n" +
            "------------------**----------\n" +
            "------------------*-----------\n" +
            "-----------------*------------\n" +
            "-----*-----------*------------\n" +
            "-----**---------*-------------\n" +
            "------*---------*-------------\n" +
            "-------*-------*--------------\n" +
            "-------**-----*---------------\n" +
            "---------*---**---------------\n" +
            "---------****-----------------\n" +
            "------------------------------\n" +
            "------------------------------\n" +
            "------------------------------\n" +
            "------------------------------\n" +
            "------------------------------\n" +
            "------------------------------\n" +
            "------------------------------\n"))
        .to_string())
    }

    #[test]
    fn cmp_strings() {
        let mut a = String::from("test");
        assert!(a == "test");
        a.push_str("/hi.jpg");
        assert!(a == "test/hi.jpg");
    }

    #[test]
    fn visit_dir() {
        let path = "dev/assets/";
        let dir = PathBuf::from(path);
        if dir.is_dir() {
            if let Ok(entries) = fs::read_dir(dir) {
                let mut found_test = false;
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().into_string().unwrap();
                        if file_name == "test.jpg" {
                            found_test = true;
                            let image = match read_image(&(path.to_owned() + &file_name)) {
                                Ok(img) => img.to_binary_image(|c| is_black_hsv(&c.to_hsv())),
                                Err(e) => panic!(e),
                            };
                            assert_eq!(image.to_string(), BinaryImage::from_string(&(
                                "------------------------------\n".to_owned()+
                                "------------------------------\n" +
                                "------------------------------\n" +
                                "------------------------------\n" +
                                "------------------------------\n" +
                                "----------------------*-------\n" +
                                "---------------------**-------\n" +
                                "---------------------*--------\n" +
                                "---------------------*--------\n" +
                                "--------------------*---------\n" +
                                "--------------------*---------\n" +
                                "--------------------*---------\n" +
                                "-------------------*----------\n" +
                                "------------------**----------\n" +
                                "------------------*-----------\n" +
                                "-----------------*------------\n" +
                                "-----*-----------*------------\n" +
                                "-----**---------*-------------\n" +
                                "------*---------*-------------\n" +
                                "-------*-------*--------------\n" +
                                "-------**-----*---------------\n" +
                                "---------*---**---------------\n" +
                                "---------****-----------------\n" +
                                "------------------------------\n" +
                                "------------------------------\n" +
                                "------------------------------\n" +
                                "------------------------------\n" +
                                "------------------------------\n" +
                                "------------------------------\n" +
                                "------------------------------\n"))
                            .to_string())                    
                        }
                    }
                }
                assert!(found_test);
            } else {
                panic!("Cannot access directory.")
            }
        }
    }
}