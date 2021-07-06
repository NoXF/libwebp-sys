extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let vendor = manifest_dir.join("vendor");

    if !vendor.join("src").exists() {
        panic!("{} dir is missing files. Try running: `git submodule update --init --recursive`", vendor.display());
    }

    let mut cc = cc::Build::new();
    cc.include(&vendor);
    cc.define("NDEBUG", Some("1"));
    cc.define("_THREAD_SAFE", Some("1"));
    cc.flag("-fvisibility=hidden"); // FIXME: msvc?

    let target = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");
    match target.as_str() {
        "x86_64" | "i686" => {
            cc.define("WEBP_HAVE_SSE2", Some("1"));
        }
        "aarch64" => {
            cc.define("WEBP_HAVE_NEON", Some("1"));
        }
        _ => {}
    };

    let files = [
    // dec
        "src/dec/alpha_dec.c",
        "src/dec/buffer_dec.c",
        "src/dec/frame_dec.c",
        "src/dec/idec_dec.c",
        "src/dec/io_dec.c",
        "src/dec/quant_dec.c",
        "src/dec/tree_dec.c",
        "src/dec/vp8_dec.c",
        "src/dec/vp8l_dec.c",
        "src/dec/webp_dec.c",

    // demux
        "src/demux/anim_decode.c",
        "src/demux/demux.c",

    // dsp_dec
        "src/dsp/alpha_processing.c",
        "src/dsp/alpha_processing_mips_dsp_r2.c",
        "src/dsp/alpha_processing_neon.c", // FIXME: .c.neon
        "src/dsp/alpha_processing_sse2.c",
        "src/dsp/alpha_processing_sse41.c",
        "src/dsp/cpu.c",
        "src/dsp/dec.c",
        "src/dsp/dec_clip_tables.c",
        "src/dsp/dec_mips32.c",
        "src/dsp/dec_mips_dsp_r2.c",
        "src/dsp/dec_msa.c",
        "src/dsp/dec_neon.c", // FIXME: .c.neon
        "src/dsp/dec_sse2.c",
        "src/dsp/dec_sse41.c",
        "src/dsp/filters.c",
        "src/dsp/filters_mips_dsp_r2.c",
        "src/dsp/filters_msa.c",
        "src/dsp/filters_neon.c", // FIXME: .c.neon
        "src/dsp/filters_sse2.c",
        "src/dsp/lossless.c",
        "src/dsp/lossless_mips_dsp_r2.c",
        "src/dsp/lossless_msa.c",
        "src/dsp/lossless_neon.c", // FIXME: .c.neon
        "src/dsp/lossless_sse2.c",
        "src/dsp/rescaler.c",
        "src/dsp/rescaler_mips32.c",
        "src/dsp/rescaler_mips_dsp_r2.c",
        "src/dsp/rescaler_msa.c",
        "src/dsp/rescaler_neon.c", // FIXME: .c.neon
        "src/dsp/rescaler_sse2.c",
        "src/dsp/upsampling.c",
        "src/dsp/upsampling_mips_dsp_r2.c",
        "src/dsp/upsampling_msa.c",
        "src/dsp/upsampling_neon.c", // FIXME: .c.neon
        "src/dsp/upsampling_sse2.c",
        "src/dsp/upsampling_sse41.c",
        "src/dsp/yuv.c",
        "src/dsp/yuv_mips32.c",
        "src/dsp/yuv_mips_dsp_r2.c",
        "src/dsp/yuv_neon.c", // FIXME: .c.neon
        "src/dsp/yuv_sse2.c",
        "src/dsp/yuv_sse41.c",

    // dsp_enc
        "src/dsp/cost.c",
        "src/dsp/cost_neon.c",
        "src/dsp/cost_mips32.c",
        "src/dsp/cost_mips_dsp_r2.c",
        "src/dsp/cost_sse2.c",
        "src/dsp/enc.c",
        "src/dsp/enc_mips32.c",
        "src/dsp/enc_mips_dsp_r2.c",
        "src/dsp/enc_msa.c",
        "src/dsp/enc_neon.c", // FIXME: .c.neon
        "src/dsp/enc_sse2.c",
        "src/dsp/enc_sse41.c",
        "src/dsp/lossless_enc.c",
        "src/dsp/lossless_enc_mips32.c",
        "src/dsp/lossless_enc_mips_dsp_r2.c",
        "src/dsp/lossless_enc_msa.c",
        "src/dsp/lossless_enc_neon.c", // FIXME: .c.neon
        "src/dsp/lossless_enc_sse2.c",
        "src/dsp/lossless_enc_sse41.c",
        "src/dsp/ssim.c",
        "src/dsp/ssim_sse2.c",

    // enc
        "src/enc/alpha_enc.c",
        "src/enc/analysis_enc.c",
        "src/enc/backward_references_cost_enc.c",
        "src/enc/backward_references_enc.c",
        "src/enc/config_enc.c",
        "src/enc/cost_enc.c",
        "src/enc/filter_enc.c",
        "src/enc/frame_enc.c",
        "src/enc/histogram_enc.c",
        "src/enc/iterator_enc.c",
        "src/enc/near_lossless_enc.c",
        "src/enc/picture_enc.c",
        "src/enc/picture_csp_enc.c",
        "src/enc/picture_psnr_enc.c",
        "src/enc/picture_rescale_enc.c",
        "src/enc/picture_tools_enc.c",
        "src/enc/predictor_enc.c",
        "src/enc/quant_enc.c",
        "src/enc/syntax_enc.c",
        "src/enc/token_enc.c",
        "src/enc/tree_enc.c",
        "src/enc/vp8l_enc.c",
        "src/enc/webp_enc.c",

    // mux
        "src/mux/anim_encode.c",
        "src/mux/muxedit.c",
        "src/mux/muxinternal.c",
        "src/mux/muxread.c",

    // utils_dec
        "src/utils/bit_reader_utils.c",
        "src/utils/color_cache_utils.c",
        "src/utils/filters_utils.c",
        "src/utils/huffman_utils.c",
        "src/utils/quant_levels_dec_utils.c",
        "src/utils/random_utils.c",
        "src/utils/rescaler_utils.c",
        "src/utils/thread_utils.c",
        "src/utils/utils.c",

    // utils_enc
        "src/utils/bit_writer_utils.c",
        "src/utils/huffman_encode_utils.c",
        "src/utils/quant_levels_utils.c",
    ];
    for f in files.iter() {
        cc.file(vendor.join(f));
    }

    cc.compile("webpsys");
}
