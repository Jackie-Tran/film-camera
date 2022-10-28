use ::image::io::Reader as ImageReader;
use image::{
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, Rgba, RgbaImage,
};
use rand;
use rand_distr::{Distribution, Normal};
use std::{env, ptr::null};

fn create_gaussian_noise(
    mean: f64,
    sd: f64,
    width: u32,
    height: u32,
    grayscale: bool,
) -> DynamicImage {
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
    return DynamicImage::ImageRgba8(noise);
}

fn calculate_noisey_pixel(image: Rgba<u8>, noise: Rgba<u8>) -> Rgba<u8> {
    let mut noisey_pixel = [0, 0, 0, 0];
    for i in 0..4 {
        // We are adding
        if image[i] > 255 - noise[i] as u8 {
            noisey_pixel[i] = 255;
        } else {
            noisey_pixel[i] = image[i] + noise[i] as u8;
        }
    }
    return Rgba::from(noisey_pixel);
}

fn apply_noise(image: &mut DynamicImage, noise: DynamicImage) {
    let width = image.width();
    let height = image.height();
    let image_size = width * height;
    assert!(image_size == noise.width() * noise.height());

    // Go through each pixel and apply noise
    for y in 0..height {
        for x in 0..width {
            let noisey_pixel = calculate_noisey_pixel(image.get_pixel(x, y), noise.get_pixel(x, y));
            image.put_pixel(x, y, noisey_pixel);
        }
    }
}

fn apply_screen_blend(base: Rgba<u8>, top: Rgba<u8>) -> Rgba<u8> {
    let mut blend_pixel = [0, 0, 0, 255];
    for i in 0..3 {
        blend_pixel[i] = (255.0
            - ((255 - base[i]) as f32 / 255.0) * ((255 - top[i]) as f32 / 255.0) * 255.0)
            as u8;
    }
    return Rgba::from(blend_pixel);
}

fn apply_film_dust(image: &mut DynamicImage) {
    // Scale film dust to image size
    let dust = ImageReader::open("./src/res/film_dust.jpg").expect("film_dust.jpg");
    let dust = dust.decode().expect("decode film_dust.jpg").resize_exact(
        image.width(),
        image.height(),
        image::imageops::FilterType::Triangle,
    );

    // Blend the pixels
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.put_pixel(
                x,
                y,
                apply_screen_blend(image.get_pixel(x, y), dust.get_pixel(x, y)),
            );
        }
    }
}

fn apply_solar_flare(image: &mut DynamicImage) {
    // Scale solar flare to image size
    let solar_flare = ImageReader::open("./src/res/solar_flare.jpg").expect("solar_flare.jpg");
    let solar_flare = solar_flare
        .decode()
        .expect("decode solare_flare.jpg")
        .resize_exact(
            image.width(),
            image.height(),
            image::imageops::FilterType::Triangle,
        );
    // Blend the pixels
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.put_pixel(
                x,
                y,
                apply_screen_blend(image.get_pixel(x, y), solar_flare.get_pixel(x, y)),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut img = ImageReader::open(file_path).expect("valid image file to exist");
    let mut img = img.decode().expect("decode image");
    let noise = create_gaussian_noise(0.0, 0.08, img.width(), img.height(), true);
    apply_noise(&mut img, noise);
    apply_film_dust(&mut img);
    apply_solar_flare(&mut img);

    img.save("film_dust_image.png");
}
