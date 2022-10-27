use ::image::io::Reader as ImageReader;
use std::env;

fn create_gaussian_noise() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let img = ImageReader::open(file_path).expect("valid image file to exist");
    let mut img = img.decode().expect("decode image");
}
