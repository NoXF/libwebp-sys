#![allow(non_snake_case)]
// bindgen --default-enum-style=rust --distrust-clang-mangling --whitelist-function='[wW][eE][bB].*' --whitelist-var='[wW][eE][bB].*' --whitelist-type='[wW][eE][bB].*' --rust-target=1.26 wrap.h -- -I./vendor > src/ffi.rs

#[allow(non_camel_case_types)]
mod ffi;
pub use ffi::*;

pub fn WebPMuxNew() -> *mut WebPMux {
    unsafe { WebPNewInternal(WEBP_MUX_ABI_VERSION) }
}

pub unsafe fn WebPInitDecoderConfig(config: *mut WebPDecoderConfig) -> bool {
    WebPInitDecoderConfigInternal(config, WEBP_DECODER_ABI_VERSION) != 0
}

impl WebPDecoderConfig {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            if WebPInitDecoderConfig(out.as_mut_ptr()) {
                Ok(out.assume_init())
            } else {
                Err(())
            }
        }
    }
}

pub unsafe fn WebPGetFeatures(
    arg1: *const u8,
    arg2: usize,
    arg3: *mut WebPBitstreamFeatures,
) -> VP8StatusCode {
    WebPGetFeaturesInternal(arg1, arg2, arg3, WEBP_DECODER_ABI_VERSION)
}

pub fn WebPDataInit(data: &mut WebPData) {
    *data = WebPData {
        bytes: std::ptr::null_mut(),
        size: 0,
    }
}

impl Default for WebPData {
    fn default() -> Self {
        Self {
            bytes: std::ptr::null(),
            size: 0,
        }
    }
}

pub unsafe fn WebPDataClear(data: &mut WebPData) {
    WebPFree(data.bytes as *mut _);
    WebPDataInit(data);
}

impl std::fmt::Debug for WebPDecBuffer__bindgen_ty_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("WebDecBuffer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;
    use std::slice;

    #[test]
    fn test_webp() {
        let mut width = 0;
        let mut height = 0;
        let mut buf = Vec::new();
        let len = File::open("./tests/test1.webp").unwrap().read_to_end(&mut buf).unwrap();

        unsafe { WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height); }
        assert!(width == 1000);
        assert!(height == 1000);
    }

    #[test]
    fn test_webp_encode_lossless() {
        let mut buf = Vec::new();
        let len = File::open("./tests/test1_1000x1000.bif").unwrap().read_to_end(&mut buf).unwrap();
        assert_eq!(4000000, len);

        let mut out_buf = std::ptr::null_mut();
        unsafe {
            let l = WebPEncodeLosslessRGBA(buf.as_ptr(), 1000, 1000, 1000 * 4, &mut out_buf);
            let out = slice::from_raw_parts(out_buf, l);

            assert_eq!(b"RIFF", &out[0..4]);
            assert_eq!(b"WEBP", &out[8..12]);
        }

    }

    #[test]
    fn test_webp_encode() {
        let mut buf = Vec::new();
        let len = File::open("./tests/test1_1000x1000.bif").unwrap().read_to_end(&mut buf).unwrap();
        assert_eq!(4000000, len);

        let mut out_buf = std::ptr::null_mut();
        unsafe {
            let l = WebPEncodeRGBA(buf.as_ptr(), 1000, 1000, 1000 * 4, 90 as f32, &mut out_buf);
            let out = slice::from_raw_parts(out_buf, l);

            assert_eq!(b"RIFF", &out[0..4]);
            assert_eq!(b"WEBP", &out[8..12]);
        }
    }

    #[test]
    fn test_webp_decode() {
        let mut buf = Vec::new();
        let len = File::open("./tests/test1.webp").unwrap().read_to_end(&mut buf).unwrap();
        let mut width = 0;
        let mut height = 0;

        unsafe {
            WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height);
            assert!(width == 1000);
            assert!(height == 1000);

            let decode_buf = WebPDecodeRGBA(buf.as_ptr(), len, &mut width, &mut height);

            let mut out_buf = std::ptr::null_mut();
            let l = WebPEncodeRGBA(decode_buf, width, height, width * 4, 90 as f32, &mut out_buf);
            let out = slice::from_raw_parts(out_buf, l);

            assert_eq!(b"RIFF", &out[0..4]);
            assert_eq!(b"WEBP", &out[8..12]);
        }
    }

    #[test]
    fn config_debug() {
        let _ = format!("{:?}", WebPDecoderConfig::new().unwrap());
    }

    #[test]
    fn poke() {
        unsafe {
            assert_eq!(66048, WebPGetEncoderVersion());

            let mut data = ::std::ptr::null_mut();
            let rgb = [1u8, 2, 3];
            let size = WebPEncodeRGB(rgb.as_ptr(), 1, 1, 1, 50., &mut data);
            assert!(size > 0);
            assert!(!data.is_null());
            let mut w = 0;
            let mut h = 0;
            let decoded = WebPDecodeRGBA(data, size, &mut w, &mut h);
            assert_eq!(1, w);
            assert_eq!(1, h);
            assert!(!decoded.is_null());
            WebPFree(data as *mut _);
            WebPFree(decoded as *mut _);
        }

    }
}
