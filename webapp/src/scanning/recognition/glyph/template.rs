use std::path::PathBuf;

use visioncortex::ColorImage;

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
    use crate::scanning::is_black;

    use super::*;

    #[test]
    fn test_read_image() {
        let image = match read_image("dev/assets/test.jpg") {
            Ok(img) => img.to_binary_image(|c| {println!("{} {} {}", c.to_hsv().h, c.to_hsv().s, c.to_hsv().v); c.r > 128}),
            Err(e) => panic!(e),
        };
        println!("{}", image.to_string());
        assert!(false);
    }
}