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
        blend_pixel[i] = 255 - ((255 - base[i]) / 255) * ((255 - top[i]) / 255) * 255;
    }
    return Rgba::from(blend_pixel);
}

fn apply_film_dust(image: &mut DynamicImage) {
    // Scale film dust to image size
    let dust = ImageReader::open("./screen_test_top.png").expect("film_dust.jpg");
    let dust = dust.decode().expect("decode film_dust.jpg").resize_exact(
        image.width(),
        image.height(),
        image::imageops::FilterType::Triangle,
    );

    // Blend the pixels
    for y in 0..image.height() {
        for x in 0..image.width() {
            // image.get_pixel(x, y).blend(&dust.get_pixel(x, y));
            image.put_pixel(
                x,
                y,
                apply_screen_blend(image.get_pixel(x, y), dust.get_pixel(x, y)),
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
    // Apply noise
    // apply_noise(&mut img, noise);
    // img.save("noisey_image.png");
    apply_film_dust(&mut img);

    img.save("film_dust_image.png");
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
