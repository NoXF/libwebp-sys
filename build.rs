use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(feature = "system-link") {
        let lib_name = "libwebp";
        let find_system_lib = pkg_config::Config::new()
            .statik(true)
            .probe(lib_name)
            .is_ok();

        if find_system_lib {
            println!("cargo:rustc-link-lib=static={}", lib_name);
            return;
        }
    }

    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let vendor = manifest_dir.join("vendor");

    if !vendor.join("src").exists() {
        panic!(
            "{} dir is missing files. Try running: `git submodule update --init --recursive`",
            vendor.display()
        );
    }

    let mut cc = cc::Build::new();
    let mut sharpyuv_build = cc::Build::new();
    setup_build(&mut cc, &vendor);
    setup_build(&mut sharpyuv_build, &vendor);

    for f in glob::glob("vendor/src/**/*.c")
        .expect("glob vender/src failed")
        .flatten()
    {
        cc.file(manifest_dir.join(f));
    }

    for f in glob::glob("vendor/sharpyuv/**/*.c")
        .expect("glob vender/src failed")
        .flatten()
    {
        sharpyuv_build.file(manifest_dir.join(f));
    }

    sharpyuv_build.compile("sharpyuv");
    cc.compile("webpsys");
}

fn setup_build(build: &mut cc::Build, include_dir: &PathBuf) {
    build.include(include_dir);
    build.define("NDEBUG", Some("1"));
    build.define("_THREAD_SAFE", Some("1"));
    if !build.get_compiler().is_like_msvc() {
        build.flag("-fvisibility=hidden").flag("-Wall");
    } else {
        build.flag("-D_CRT_SECURE_NO_WARNINGS");
    }

    if let Ok(target_cpu) = env::var("TARGET_CPU") {
        build.flag_if_supported(&format!("-march={}", target_cpu));
    }

    let target = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");
    match target.as_str() {
        "x86_64" | "i686" => {
            build.define("WEBP_HAVE_SSE2", Some("1"));
            if cfg!(feature = "sse41") {
                build.define("WEBP_HAVE_SSE41", Some("1"));
                build.flag_if_supported("-msse4.1");
            }
            if cfg!(feature = "avx2") {
                build.define("WEBP_HAVE_AVX2", Some("1"));
                build.flag_if_supported("-mavx2");
            }
        }
        "aarch64" => {
            if cfg!(feature = "neon") {
                build.define("WEBP_HAVE_NEON", Some("1"));
            }
        }
        _ => {}
    };
}
