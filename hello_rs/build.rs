extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::current_dir().unwrap();

    let include_dir = crate_dir.join("include");
    if !include_dir.exists() {
        std::fs::create_dir(&include_dir).unwrap();
    }

    let dest_path = include_dir.join("hello_rs.h");

    cbindgen::generate(&crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file(&dest_path);
}