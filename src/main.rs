use ::image::io::Reader as ImageReader;
use rand;
use rand_distr::{Distribution, Normal};
use std::env;

fn create_gaussian_noise(mean: f64, sd: f64, width: u32, height: u32) -> Vec<u8> {
    let normal = Normal::new(mean, sd).unwrap();
    let mut rng = rand::thread_rng();
    let mut gaussian = normal.sample_iter(&mut rng);
    let mut noise = Vec::<u8>::new();
    while noise.len() < (width * height) as usize {
        let val = (gaussian.next().unwrap().abs() * 255.0) as u8;
        noise.push(val);
    }
    return noise;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let img = ImageReader::open(file_path).expect("valid image file to exist");
    let img = img.decode().expect("decode image");
    println!("{:?} {} {} ", img.color(), img.width(), img.height());
    let noise = create_gaussian_noise(0.0, 0.1, img.width(), img.height());
    let _ = image::save_buffer(
        "./noise.png",
        &noise[0..],
        img.width(),
        img.height(),
        image::ColorType::L8,
    );
    println!("{} {:?}", img.as_bytes().len(), &img.as_bytes()[0..5]);
    println!("{} {:?}", noise.len(), &noise[0..5]);
}
