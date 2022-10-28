use ::image::io::Reader as ImageReader;
use rand;
use rand_distr::{Distribution, Normal};
use std::env;

fn create_gaussian_noise(mean: f64, sd: f64, width: u32, height: u32, grayscale: bool) -> Vec<i16> {
    let normal = Normal::new(mean, sd).unwrap();
    let mut rng = rand::thread_rng();
    let mut gaussian = normal.sample_iter(&mut rng);
    let mut noise = Vec::<i16>::new();
    while noise.len() < (width * height * 4) as usize {
        match grayscale {
            true => {
                let val = (gaussian.next().unwrap() * 255.0) as i16;
                noise.extend([val, val, val, 255])
            }
            _ => {
                for _ in 0..3 {
                    let val = (gaussian.next().unwrap() * 255.0) as i16;
                    noise.push(val);
                }
                noise.push(255);
            }
        }
    }
    return noise;
}

fn add_noise(image: &[u8], noise: &[i16]) -> Vec<u8> {
    assert!(image.len() == noise.len());
    let mut sum_image = Vec::<u8>::new();
    for i in 0..image.len() {
        if noise[i] > 0 {
            // We are adding
            if image[i] > 255 - noise[i] as u8 {
                sum_image.push(255);
            } else {
                sum_image.push(image[i] + noise[i] as u8);
            }
        } else if image[i] < -noise[i] as u8 {
            // We underflow
            sum_image.push(0);
        } else {
            // We are subtracting
            sum_image.push(image[i] - -noise[i] as u8);
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
    let noise = create_gaussian_noise(0.0, 0.08, img.width(), img.height(), true);
    // Apply noise
    let sum_image = add_noise(img.as_bytes(), &noise);
    println!("{:?}", &sum_image[0..5]);
    let _ = image::save_buffer(
        "./noisy_image.png",
        &sum_image[0..],
        img.width(),
        img.height(),
        image::ColorType::Rgba8,
    );
    println!("{} {:?}", img.as_bytes().len(), &img.as_bytes()[0..5]);
    // println!("{} {:?}", noise.len(), &noise);
}
