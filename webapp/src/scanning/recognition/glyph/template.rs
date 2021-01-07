use std::path::PathBuf;

use visioncortex::{BinaryImage, ColorImage};

pub struct GlyphLibrary {
    templates: Vec<BinaryImage>,
}

fn read_image(input_path: &str) -> Result<ColorImage, String> {
    let img = image::open(PathBuf::from(input_path));
    let img = match img {
        Ok(file) => file.to_rgba8(),
        Err(_) => return Err(String::from("No image file found at specified input path")),
    };

    let (width, height) = (img.width() as usize, img.height() as usize);
    let img = ColorImage {pixels: img.as_raw().to_vec(), width, height};

    Ok(img)
}

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
}