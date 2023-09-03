pub mod args;
pub mod image;

use crate::image::Image;

fn main() {
    let mut image = Image::new();

    image.process();
}
