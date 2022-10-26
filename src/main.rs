use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_file_contents(buffer: &mut Vec<u8>, file_path: &String) {
    let mut file = File::open(file_path).expect("error");
    // TODO: use buffer and multithreading to process file contents
    file.read_to_end(buffer).expect("Error reading file");
    println!("Read {} bytes", buffer.len());
}

fn decode_jpeg() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut file_buffer: Vec<u8> = Vec::new();
    get_file_contents(&mut file_buffer, file_path);
    for x in file_buffer[0..5].iter_mut() {
        println!("{}", x);
        if *x == 0xff {
            println!("found FF");
        }
    }
}
