use ::image::io::Reader as ImageReader;
use chrono::prelude::*;
use colored::Colorize;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rand;
use rand_distr::{Distribution, Normal};
use rusttype::{Font, Scale};
use std::{
    env,
    time::{Duration, Instant},
};

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
            let noisey_pixel = calculate_noisey_pixel(
                image::GenericImageView::get_pixel(image, x, y),
                image::GenericImageView::get_pixel(&noise, x, y),
            );
            image.put_pixel(x, y, noisey_pixel);
        }
    }
}

fn apply_screen_blend(base: Rgba<u8>, top: Rgba<u8>) -> Rgba<u8> {
    let mut blend_pixel = [0, 0, 0, 255];
    for i in 0..4 {
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
                apply_screen_blend(
                    image::GenericImageView::get_pixel(image, x, y),
                    image::GenericImageView::get_pixel(&dust, x, y),
                ),
            );
        }
    }
}

fn apply_light_leak(image: &mut DynamicImage) {
    // Scale solar flare to image size
    let light_leak = ImageReader::open("./src/res/light_leak.jpg").expect("light_leak.jpg");
    let light_leak = light_leak
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
                apply_screen_blend(
                    image::GenericImageView::get_pixel(image, x, y),
                    image::GenericImageView::get_pixel(&light_leak, x, y),
                ),
            );
        }
    }
}

fn create_timestamp() -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    let font = Vec::from(include_bytes!("./res/ds_digital_Font/DS-DIGIT.TTF") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let height = 29.0;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };

    let timestamp = Local::now().format("%m %d %Y").to_string();

    let (w, h) = text_size(scale, &font, &timestamp);
    let mut text_image = RgbaImage::new(w as u32, h as u32);
    let mut outer_glow = RgbaImage::new(w as u32, h as u32);
    draw_text_mut(
        &mut text_image,
        Rgba([255, 180, 0, 200]),
        -2,
        -2,
        scale,
        &font,
        &timestamp,
    );
    draw_text_mut(
        &mut outer_glow,
        Rgba([255, 0, 0, 100]),
        0,
        -1,
        scale,
        &font,
        &timestamp,
    );
    draw_text_mut(
        &mut outer_glow,
        Rgba([255, 0, 0, 100]),
        -3,
        -3,
        scale,
        &font,
        &timestamp,
    );

    // Apply outer glow
    for y in 0..h as u32 {
        for x in 0..w as u32 {
            text_image.put_pixel(
                x,
                y,
                apply_screen_blend(*text_image.get_pixel(x, y), *outer_glow.get_pixel(x, y)),
            );
        }
    }
    return text_image;
}

fn add_timestamp(image: &mut DynamicImage, timestamp: image::ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let offset_x = image.width() - timestamp.width() - timestamp.width() / 4;
    let offset_y = image.height() - timestamp.height() - timestamp.height();
    for y in 0..timestamp.height() {
        for x in 0..timestamp.width() {
            image.put_pixel(
                offset_x + x,
                offset_y + y,
                apply_screen_blend(
                    image.get_pixel(offset_x + x, offset_y + y),
                    *timestamp.get_pixel(x, y),
                ),
            )
        }
    }
}

fn log_duration(process: String, duration: Duration) {
    println!(
        "{} {} in {:?}s",
        "Finished".green(),
        process,
        duration.as_secs_f32()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut start = Instant::now();
    let img = ImageReader::open(file_path).expect("valid image file to exist");
    let mut img = img.decode().expect("decode image");
    let mut duration = start.elapsed();
    log_duration(format!("opening {}", file_path), duration);

    start = Instant::now();
    let noise = create_gaussian_noise(0.0, 0.08, img.width(), img.height(), false);
    apply_noise(&mut img, noise);
    duration = start.elapsed();
    log_duration("applying noise".to_string(), duration);

    start = Instant::now();
    apply_film_dust(&mut img);
    duration = start.elapsed();
    log_duration("applying film dust".to_string(), duration);

    start = Instant::now();
    apply_light_leak(&mut img);
    duration = start.elapsed();
    log_duration("applying light leak".to_string(), duration);

    start = Instant::now();
    let timestamp = create_timestamp();
    add_timestamp(&mut img, timestamp);
    duration = start.elapsed();
    log_duration("adding timestamp".to_string(), duration);

    let _ = img.save("final.png");
}
