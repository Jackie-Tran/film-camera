use fs_extra::{copy_items, dir::CopyOptions};
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut from_paths = Vec::new();
    from_paths.push("src/res");

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut options = CopyOptions::new();
    options.overwrite = true;

    copy_items(&from_paths, &out_dir, &options).unwrap();
}
