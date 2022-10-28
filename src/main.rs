use ::image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};
use rand;
use rand_distr::{Distribution, Normal};
use std::env;

fn create_gaussian_noise(
    mean: f64,
    sd: f64,
    width: u32,
    height: u32,
    grayscale: bool,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let normal = Normal::new(mean, sd).unwrap();
    let mut rng = rand::thread_rng();
    let mut gaussian = normal.sample_iter(&mut rng);
    let mut noise = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            match grayscale {
                true => {
                    let val = (gaussian.next().unwrap() * 255.0).abs() as u8;
                    noise.put_pixel(x, y, Rgba::from([val, val, val, 255]));
                }
                _ => {
                    let mut pixel = [0, 0, 0, 255];
                    for i in 0..3 {
                        let val = (gaussian.next().unwrap() * 255.0).abs() as u8;
                        pixel[i] = val;
                    }
                    noise.put_pixel(x, y, Rgba::from(pixel));
                }
            }
        }
    }
    return noise;
}

fn calculate_noisey_pixel(image: Rgba<u8>, noise: [i16; 4]) -> Rgba<u8> {
    let mut noisey_pixel = [0, 0, 0, 0];
    for i in 0..4 {
        if noise[i] > 0 {
            // We are adding
            if image[i] > 255 - noise[i] as u8 {
                noisey_pixel[i] = 255;
            } else {
                noisey_pixel[i] = image[i] + noise[i] as u8;
            }
        } else if image[i] < -noise[i] as u8 {
            // We underflow
            noisey_pixel[i] = 0;
        } else {
            // We are subtracting
            noisey_pixel[i] = image[i] - -noise[i] as u8;
        }
    }
    return Rgba::from(noisey_pixel);
}

fn add_noise(image: &mut DynamicImage, noise: &[i16]) {
    let width = image.width();
    let height = image.height();
    let image_size = (width * height * 4) as usize;
    assert!(image_size == noise.len());

    // Go through each pixel and apply noise
    for y in 0..height {
        for x in 0..width {
            let index = (width * x + y) as usize;
            let noise_pixel = [
                noise[index],
                noise[index + 1],
                noise[index + 2],
                noise[index + 3],
            ];
            let noisey_pixel = calculate_noisey_pixel(image.get_pixel(x, y), noise_pixel);
            image.put_pixel(x, y, noisey_pixel);
        }
    }
}

fn add_film_dust() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut img = ImageReader::open(file_path).expect("valid image file to exist");
    let mut img = img.decode().expect("decode image");
    let noise = create_gaussian_noise(0.0, 0.08, img.width(), img.height(), false);
    noise.save("noise.png");
    // Apply noise
    // let noisey_image = add_noise(&mut img, &noise);
    // img.save("./noisey_image.png");
    // println!("{:?}", &img.as_bytes()[0..5]);
    // println!("{:?}", &noisey_image[0..5]);

    // Scale film dust to image size
    // let dust = ImageReader::open("./src/res/film_dust.jpg").expect("film_dust.jpg");
    // let dust = dust.decode().expect("decode film_dust.jpg").resize_exact(
    //     img.width(),
    //     img.height(),
    //     image::imageops::FilterType::Triangle,
    // );

    // let _ = image::save_buffer(
    //     "./resized film dust.png",
    //     dust.as_bytes(),
    //     img.width(),
    //     img.height(),
    //     image::ColorType::Rgb8,
    // );

    // let _ = image::save_buffer(
    //     "./noisy_image.png",
    //     &noisey_image[0..],
    //     img.width(),
    //     img.height(),
    //     image::ColorType::Rgba8,
    // );
    // println!("{} {:?}", img.as_bytes().len(), &img.as_bytes()[0..5]);
    // println!("{} {:?}", noise.len(), &noise);
}
