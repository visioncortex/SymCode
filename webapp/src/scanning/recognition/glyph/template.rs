use std::{fs, path::PathBuf};

use visioncortex::{BinaryImage, ColorImage, Sampler};
use web_sys::console;

use crate::{scanning::{image_diff_area, is_black}};

use super::GlyphCode;

#[derive(Clone, Copy, Debug)]
pub enum GlyphLabel {
    Empty = 0,
    Up,
    Right,
    Down,
    Left,
}

impl Default for GlyphLabel {
    fn default() -> Self {
        Self::Empty
    }
}

impl GlyphLabel {
    /// Will be replaced by sth like FromPrimitive
    fn from_usize_representation(label: usize) -> Self {
        match label {
            0 => Self::Empty,
            1 => Self::Up,
            2 => Self::Right,
            3 => Self::Down,
            4 => Self::Left,
            _ => panic!("GlyphLabel representation ".to_owned() + &label.to_string() + " is not defined!"),
        }
    }
}

pub struct GlyphLibrary {
    templates: Vec<(BinaryImage, GlyphLabel)>,
}

impl Default for GlyphLibrary {
    fn default() -> Self {
        Self { templates: vec![] }
    }
}

impl GlyphLibrary {
    /// Takes the binary image of the template and the usize representation of the label
    pub(crate) fn add_template(&mut self, image: BinaryImage, label: usize) {
        //console::log_1(&format!("{}\n{}", label, image.to_string()).into());
        self.templates.push((image, GlyphLabel::from_usize_representation(label)));
    }

    pub(crate) fn find_most_similar_glyph(&self, image: BinaryImage) -> GlyphLabel {
        let size = GlyphCode::GLYPH_SIZE;
        let image = &Sampler::resample_image(&image, size, size);

        self.templates.iter()
            .fold( (image_diff_area(&self.templates[0].0, image), self.templates[0].1),
                |(min_error, min_label), (template, label)| {
                    let error = image_diff_area(template, image);
                    if error < min_error {
                        (error, *label)
                    } else {
                        (min_error, min_label)
                    }
                }).1
    }
}

// For CMDAPP
impl GlyphLibrary {
    const DEFAULT_DIR: &'static str = "./";

    /// Loads the glyph templates in the specified directory as BinaryImage.
    ///
    /// Panics if path is not found or no jpg is found there.
    pub fn load_from_directory(path: &str) -> Self {
        let mut path = String::from(path);

        if !path.ends_with('/') {
            path.push_str("/".into());
        }
        let dir = PathBuf::from(path.clone());
        console::log_1(&format!("{:?}", dir).into());
        if !dir.is_dir() {
            panic!("GlyphLibrary Error: Specified path ".to_owned() + &path + " is not a directory.");
        }

        if let Ok(entries) = fs::read_dir(dir) { // Read the directory
            Self {
                templates: entries.into_iter().filter_map(|entry| { // Read each entry in the directory
                        if let Ok(file) = entry {
                            // Read the image in the entry
                            let file_name = file.file_name().into_string().unwrap();
                            println!("{}", &(path.to_owned() + &file_name));
                            Some(
                                match read_image(&(path.clone() + &file_name)) {
                                    Ok(img) => (img.to_binary_image(|c| is_black(&c.to_hsv())), GlyphLabel::Empty), // Dummy label for category: figure it out later
                                    Err(e) => {
                                        //console::log_1(&e.into());
                                        return None;
                                    },
                                }
                            )
                        } else {
                            None
                        }
                    }).collect()
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

    use crate::scanning::is_black;

    use super::*;

    #[test]
    fn test_read_image() {
        let image = match read_image("dev/assets/test.jpg") {
            Ok(img) => img.to_binary_image(|c| is_black(&c.to_hsv())),
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
                                Ok(img) => img.to_binary_image(|c| is_black(&c.to_hsv())),
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

    #[test]
    fn load_default_templates() {
        let library = GlyphLibrary::default();
        assert_eq!(library.templates.len(), 4);
    }
}