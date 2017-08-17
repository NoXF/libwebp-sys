extern crate bindgen;
extern crate cmake;
use std::env;
use std::path::PathBuf;
use bindgen::Builder;
use cmake::Config;

fn main() {
    let dst = Config::new("libwebp").build_target("webp").build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=webp");
}
