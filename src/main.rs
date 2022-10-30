use ::image::io::Reader as ImageReader;
use clap::Parser;
use colored::Colorize;
use std::{
    env,
    time::{Duration, Instant},
};
pub mod filters;

fn log_duration(process: String, duration: Duration) {
    println!(
        "{} {} in {:?}s",
        "Finished".green(),
        process,
        duration.as_secs_f32()
    );
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Standard deviation for gaussian distribution
    #[arg(short, long, default_value_t = 0.08)]
    noise_intensity: f32,

    /// Use grayscale noise
    #[arg(short, long, default_value_t = false)]
    grayscale_noise: bool,

    /// Add timestamp to final image
    #[arg(short, long, default_value_t = false)]
    timestamp: bool,

    /// Output destination
    #[arg(short, long)]
    output: String,

    /// Input image
    input_file: String,
}
fn main() {
    let cli = Cli::parse();

    let dust_image_path = concat!(env!("OUT_DIR"), "/res/film_dust.jpg");
    let light_leak_path = concat!(env!("OUT_DIR"), "/res/light_leak.jpg");
    let font_path = concat!(env!("OUT_DIR"), "/res/DS-DIGIT.TTF");

    let mut start = Instant::now();
    let img = ImageReader::open(cli.input_file.clone()).expect("input image file to exist");
    let mut img = img.decode().expect("decode image");
    let mut duration = start.elapsed();
    log_duration(format!("opening {}", cli.input_file), duration);

    start = Instant::now();
    filters::add_noise(&mut img, cli.noise_intensity, cli.grayscale_noise);
    duration = start.elapsed();
    log_duration("applying noise".to_string(), duration);

    start = Instant::now();
    filters::add_film_dust(&mut img, dust_image_path);
    duration = start.elapsed();
    log_duration("applying film dust".to_string(), duration);

    start = Instant::now();
    filters::add_light_leak(&mut img, light_leak_path);
    duration = start.elapsed();
    log_duration("applying light leak".to_string(), duration);

    if cli.timestamp {
        start = Instant::now();
        filters::add_timestamp(&mut img, font_path);
        duration = start.elapsed();
        log_duration("adding timestamp".to_string(), duration);
    }

    let _ = img.save(cli.output);
}
