#[cfg(not(feature = "system-dylib"))]
use std::env;
use std::path::PathBuf;

#[cfg(feature = "system-dylib")]
const PACKAGE_CONFIG_NAME: &str = "libwebp";

fn main() {
    #[cfg(feature = "generate-bindings")]
    generate_bindings();

    #[cfg(all(not(feature = "generate-bindings"), feature = "system-dylib"))]
    pkg_config::Config::new()
        .probe(PACKAGE_CONFIG_NAME)
        .unwrap();

    #[cfg(not(feature = "system-dylib"))]
    do_build();
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    println!("cargo:rerun-if-changed=wrap.h");
    let bindings = bindgen::Builder::default()
        .header("wrap.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .trust_clang_mangling(false)
        .clang_args(
            include_paths()
                .into_iter()
                .map(|include_path| format!("-I{}", include_path.to_str().unwrap())),
        )
        .impl_debug(true)
        .allowlist_function("[wW][eE][bB].*")
        .allowlist_var("[wW][eE][bB].*")
        .allowlist_type("[wW][eE][bB].*")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/ffi.rs")
        .expect("Couldn't write bindings to ffi.rs");
}

#[cfg(all(feature = "system-dylib", feature = "generate-bindings"))]
fn include_paths() -> Vec<PathBuf> {
    let system_libwebp = pkg_config::Config::new()
        .probe(PACKAGE_CONFIG_NAME)
        .unwrap();
    system_libwebp.include_paths
}

#[cfg(all(not(feature = "system-dylib"), feature = "generate-bindings"))]
fn include_paths() -> Vec<PathBuf> {
    vec![PathBuf::from("vendor/src")]
}

#[cfg(not(feature = "system-dylib"))]
fn do_build() {
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
    setup_build(&mut cc, &vendor);

    for f in glob::glob("vendor/src/**/*.c")
        .expect("glob vender/src failed")
        .flatten()
    {
        cc.file(manifest_dir.join(f));
    }

    for f in glob::glob("vendor/sharpyuv/**/*.c").expect("glob vendor/src failed") {
        let f = f.expect("glob iteration vendor/src failed");
        cc.file(manifest_dir.join(f));
    }
    cc.compile("webpsys");
}

#[cfg(not(feature = "system-dylib"))]
fn setup_build(build: &mut cc::Build, include_dir: &PathBuf) {
    build.include(include_dir);
    build.define("NDEBUG", Some("1"));
    build.define("_THREAD_SAFE", Some("1"));
    if !build.get_compiler().is_like_msvc() {
        build.flag("-fvisibility=hidden").flag("-Wall");
    } else {
        build.flag("-D_CRT_SECURE_NO_WARNINGS");
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");
    let target_features = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let has_feature = |f: &str| target_features.split(',').any(|feature| feature == f);

    let target_cpu = env::var("TARGET_CPU").ok();
    let target_cpu = target_cpu.as_deref().unwrap_or(&*target_arch);
    build.flag_if_supported(&format!("-march={target_cpu}"));

    match target_arch.as_str() {
        "x86_64" | "i686" => {
            build.define("WEBP_HAVE_SSE2", Some("1"));
            if cfg!(feature = "sse41") || has_feature("sse4.1") {
                build.define("WEBP_HAVE_SSE41", Some("1"));
                build.flag_if_supported("-msse4.1");
            }
            if cfg!(feature = "avx2") || has_feature("avx2") {
                build.define("WEBP_HAVE_AVX2", Some("1"));
                build.flag_if_supported("-mavx2");
            }
        }
        "aarch64" => {
            if cfg!(feature = "neon") || has_feature("neon") {
                build.define("WEBP_HAVE_NEON", Some("1"));
            }

            // If any optimizations are ennabled, we must remove -gdwarf flags
            // Which we can only do by stopping debug mode entirely since we can't
            // fix env flags.
            let gccflags = build
                .get_compiler()
                .cflags_env()
                .to_string_lossy()
                .to_string();

            if gccflags.contains("-O0")
                || gccflags.contains("-O1")
                || gccflags.contains("-O2")
                || gccflags.contains("-O3")
                || gccflags.contains("-Ofast")
                || gccflags.contains("-Os")
                || gccflags.contains("--opt-level=0")
                || gccflags.contains("--opt-level=1")
                || gccflags.contains("--opt-level=2")
                || gccflags.contains("--opt-level=3")
                || gccflags.contains("--opt-level=s")
                || gccflags.contains("--opt-level=z")
            {
                build.debug(false);
            }
        }
        _ => {}
    };
}
