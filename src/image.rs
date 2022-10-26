pub enum ImageFileType {
    PNG,
    JPEG,
}

fn unpack(bytes: &[u8]) -> i32 {
    let mut res: i32 = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        // println!("{} {}", i, byte);
        res = res * 256 + byte as i32;
    }
    println!("{:2X?}", res);
    return res;
}

pub fn is_png(data: &[u8]) -> bool {
    if unpack(&data[0..1]) != 0x89 {
        return false;
    }

    if unpack(&data[1..4]) != 0x504e47 {
        return false;
    }

    if unpack(&data[4..8]) != 0x0d0a1a0a {
        return false;
    }

    return true;
}

pub fn decode_png(data: &Vec<u8>) {
    // Check if file is png 89 50 4E 47 0D 0A 1A 0A
    println!("{:?}", &data[0..8]);

    if is_png(&data[0..8]) {
        println!("is png!")
    }
    // TODO: read image data
}
