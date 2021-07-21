// Get an image from the clipboard and write it to
// stdout as PNG.
// Bart Massey 2021

use std::convert::TryInto;
use std::io::Write;

use arboard::*;
use image::*;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let image = match clipboard.get_image() {
        Ok(img) => img,
        Err(e) => {
            eprintln!("error getting image: {}", e);
            return;
        }
    };
    eprintln!("getting {}×{} image", image.width, image.height);

    let image: RgbaImage = ImageBuffer::from_raw(
        image.width.try_into().unwrap(),
        image.height.try_into().unwrap(),
        image.bytes.into_owned(),
    ).unwrap();
    let mut stdout = std::io::stdout();
    let image = DynamicImage::ImageRgba8(image);
    image.write_to(&mut stdout, ImageOutputFormat::Png).unwrap();
    stdout.flush().unwrap();
}
