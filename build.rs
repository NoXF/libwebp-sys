extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("libwebp").build_target("webp").build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=webp");
}
