use ::image::io::Reader as ImageReader;
use rand;
use rand_distr::{Distribution, Normal};
use std::env;

fn create_gaussian_noise(mean: f64, sd: f64, width: u32, height: u32) -> Vec<u8> {
    let normal = Normal::new(mean, sd).unwrap();
    let mut rng = rand::thread_rng();
    let mut gaussian = normal.sample_iter(&mut rng);
    let mut noise = Vec::<u8>::new();
    while noise.len() < (width * height * 4) as usize {
        for i in (0..3) {
            let val = (gaussian.next().unwrap() * 255.0) as u8;
            noise.push(val);
        }
        noise.push(255);
    }
    return noise;
}

fn add_two_images(image1: &[u8], image2: &[u8]) -> Vec<u8> {
    assert!(image1.len() == image2.len());
    let mut sum_image = Vec::<u8>::new();
    for i in 0..image1.len() {
        if image1[i] > std::u8::MAX - image2[i] {
            sum_image.push(255);
        } else if image1[i] < image2[i] {
            sum_image.push(0);
        } else {
            sum_image.push(image1[i] + image2[i]);
        }
    }
    return sum_image;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let img = ImageReader::open(file_path).expect("valid image file to exist");
    let img = img.decode().expect("decode image");
    println!("{:?} {} {} ", img.color(), img.width(), img.height());
    let noise = create_gaussian_noise(0.0, 0.2, img.width(), img.height());
    let _ = image::save_buffer(
        "./noise.png",
        &noise[0..],
        img.width(),
        img.height(),
        image::ColorType::Rgba8,
    );
    // Apply noise
    let sum_image = add_two_images(img.as_bytes(), &noise);
    println!("{:?}", &sum_image[0..5]);
    let _ = image::save_buffer(
        "./noisy_image.png",
        &sum_image[0..],
        img.width(),
        img.height(),
        image::ColorType::Rgba8,
    );
    println!("{} {:?}", img.as_bytes().len(), &img.as_bytes()[0..5]);
    println!("{} {:?}", noise.len(), &noise[0..5]);
}
