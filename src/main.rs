use convert_image_rs::image::Image;

fn main() {
    let mut image = Image::new();

    image.scale();
    image.save();
}
