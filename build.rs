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

    let bindings = Builder::default()
        .header("wrapper.h")
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    println!("bindgs-generate-bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("out_dir: {:?}", out_path);
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Couldn't write bindings!");
}
