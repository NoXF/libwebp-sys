#![allow(non_snake_case)]
// bindgen --default-enum-style=rust --distrust-clang-mangling --whitelist-function='[wW][eE][bB].*' --whitelist-var='[wW][eE][bB].*' --whitelist-type='[wW][eE][bB].*' --size_t-is-usize wrap.h -- -I./vendor > src/ffi.rs

#[allow(non_camel_case_types)]
mod ffi;
pub use ffi::*;

const WEBP_DEMUX_ABI_VERSION: std::os::raw::c_int =
    ffi::WEBP_DEMUX_ABI_VERSION as std::os::raw::c_int;
const WEBP_MUX_ABI_VERSION: std::os::raw::c_int = ffi::WEBP_MUX_ABI_VERSION as std::os::raw::c_int;
const WEBP_DECODER_ABI_VERSION: std::os::raw::c_int =
    ffi::WEBP_DECODER_ABI_VERSION as std::os::raw::c_int;

pub fn WebPMuxNew() -> *mut WebPMux {
    unsafe { WebPNewInternal(WEBP_MUX_ABI_VERSION) }
}

pub fn WebPGetMuxABIVersion() -> std::os::raw::c_int {
    WEBP_MUX_ABI_VERSION
}

pub fn WebPGetDemuxABIVersion() -> std::os::raw::c_int {
    WEBP_DEMUX_ABI_VERSION
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

pub unsafe fn WebPInitConfig(config: *mut WebPConfig) -> bool {
    WebPConfigInitInternal(
        config,
        WebPPreset::WEBP_PRESET_DEFAULT,
        75.0,
        WEBP_DECODER_ABI_VERSION,
    ) != 0
}

impl WebPConfig {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            if WebPInitConfig(out.as_mut_ptr()) {
                Ok(out.assume_init())
            } else {
                Err(())
            }
        }
    }

    pub fn new_with_preset(preset: WebPPreset, quality: f32) -> Result<Self, ()> {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            if WebPConfigInitInternal(out.as_mut_ptr(), preset, quality, WEBP_DECODER_ABI_VERSION)
                != 0
            {
                Ok(out.assume_init())
            } else {
                Err(())
            }
        }
    }
}

pub unsafe fn WebPPictureInit(config: *mut WebPPicture) -> bool {
    WebPPictureInitInternal(config, WEBP_DECODER_ABI_VERSION) != 0
}

impl WebPPicture {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            if WebPPictureInit(out.as_mut_ptr()) {
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
    use std::fs::File;
    use std::io::prelude::*;
    use std::slice;

    #[test]
    fn test_webp() {
        let mut width = 0;
        let mut height = 0;
        let mut buf = Vec::new();
        let len = File::open("./tests/test1.webp")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();

        unsafe {
            WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height);
        }
        assert!(width == 1000);
        assert!(height == 1000);
    }

    #[test]
    fn test_webp_encode_lossless() {
        let mut buf = Vec::new();
        let len = File::open("./tests/test1_1000x1000.bif")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
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
        let len = File::open("./tests/test1_1000x1000.bif")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        assert_eq!(4000000, len);

        let mut out_buf = std::ptr::null_mut();
        unsafe {
            let l = WebPEncodeRGBA(buf.as_ptr(), 1000, 1000, 1000 * 4, 90_f32, &mut out_buf);
            let out = slice::from_raw_parts(out_buf, l);

            assert_eq!(b"RIFF", &out[0..4]);
            assert_eq!(b"WEBP", &out[8..12]);
        }
    }

    #[test]
    fn test_webp_encode_advanced() {
        use std::ffi::c_void;

        let mut buf = Vec::new();
        let len = File::open("./tests/test1_1000x1000.bif")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        assert_eq!(4000000, len);

        unsafe {
            let config = WebPConfig::new().unwrap();
            let mut picture = WebPPicture::new().unwrap();
            picture.use_argb = 1;
            picture.argb = buf.as_ptr() as *mut u32;
            picture.width = 1000;
            picture.height = 1000;
            picture.argb_stride = 1000;

            let mut out = vec![];

            unsafe extern "C" fn writer_function(
                data: *const u8,
                data_size: usize,
                picture: *const WebPPicture,
            ) -> ::std::os::raw::c_int {
                let out: &mut Vec<u8> = &mut *((*picture).custom_ptr as *mut std::vec::Vec<u8>);
                out.extend_from_slice(std::slice::from_raw_parts(data, data_size));
                0
            }

            picture.writer = Some(writer_function);
            picture.custom_ptr = &mut out as *mut _ as *mut c_void;

            assert_eq!(WebPEncode(&config, &mut picture), 0);

            assert_eq!(b"RIFF", &out[0..4]);
            assert_eq!(b"WEBP", &out[8..12]);
        }
    }

    #[test]
    fn test_webp_decode() {
        let mut buf = Vec::new();
        let len = File::open("./tests/test1.webp")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        let mut width = 0;
        let mut height = 0;

        unsafe {
            WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height);
            assert!(width == 1000);
            assert!(height == 1000);

            let decode_buf = WebPDecodeRGBA(buf.as_ptr(), len, &mut width, &mut height);

            let mut out_buf = std::ptr::null_mut();
            let l = WebPEncodeRGBA(decode_buf, width, height, width * 4, 90_f32, &mut out_buf);
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
            assert_eq!(66052, WebPGetEncoderVersion());

            let mut data = ::std::ptr::null_mut();
            let rgb = [1u8, 2, 3];
            // `stride` corresponds to the number of bytes needed to jump from one row to the next.
            // For RGB, this is 3 * width.
            // For RGBA, this is 4 * width.
            let size = WebPEncodeRGB(rgb.as_ptr(), 1, 1, 3, 50., &mut data);
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
