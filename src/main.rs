pub mod image;
pub mod args;

use crate::image::Image;

fn main() {
    let mut image = Image::new();

    image.process();
}