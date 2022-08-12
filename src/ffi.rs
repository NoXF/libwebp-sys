/* automatically generated by rust-bindgen */

pub const WEBP_ENCODER_ABI_VERSION: ::std::os::raw::c_int = 527;
pub const WEBP_MAX_DIMENSION: ::std::os::raw::c_int = 16383;
pub const WEBP_DECODER_ABI_VERSION: ::std::os::raw::c_int = 521;
pub const WEBP_MUX_ABI_VERSION: ::std::os::raw::c_int = 264;
extern "C" {
    pub fn WebPMalloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn WebPFree(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn WebPGetEncoderVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPEncodeRGB(
        rgb: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        quality_factor: f32,
        output: *mut *mut u8,
    ) -> usize;
}
extern "C" {
    pub fn WebPEncodeBGR(
        bgr: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        quality_factor: f32,
        output: *mut *mut u8,
    ) -> usize;
}
extern "C" {
    pub fn WebPEncodeRGBA(
        rgba: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        quality_factor: f32,
        output: *mut *mut u8,
    ) -> usize;
}
extern "C" {
    pub fn WebPEncodeBGRA(
        bgra: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        quality_factor: f32,
        output: *mut *mut u8,
    ) -> usize;
}
extern "C" {
    pub fn WebPEncodeLosslessRGB(
        rgb: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        output: *mut *mut u8,
    ) -> usize;
}
extern "C" {
    pub fn WebPEncodeLosslessBGR(
        bgr: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        output: *mut *mut u8,
    ) -> usize;
}
extern "C" {
    pub fn WebPEncodeLosslessRGBA(
        rgba: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        output: *mut *mut u8,
    ) -> usize;
}
extern "C" {
    pub fn WebPEncodeLosslessBGRA(
        bgra: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        output: *mut *mut u8,
    ) -> usize;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPImageHint {
    WEBP_HINT_DEFAULT = 0,
    WEBP_HINT_PICTURE = 1,
    WEBP_HINT_PHOTO = 2,
    WEBP_HINT_GRAPH = 3,
    WEBP_HINT_LAST = 4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPConfig {
    pub lossless: ::std::os::raw::c_int,
    pub quality: f32,
    pub method: ::std::os::raw::c_int,
    pub image_hint: WebPImageHint,
    pub target_size: ::std::os::raw::c_int,
    pub target_PSNR: f32,
    pub segments: ::std::os::raw::c_int,
    pub sns_strength: ::std::os::raw::c_int,
    pub filter_strength: ::std::os::raw::c_int,
    pub filter_sharpness: ::std::os::raw::c_int,
    pub filter_type: ::std::os::raw::c_int,
    pub autofilter: ::std::os::raw::c_int,
    pub alpha_compression: ::std::os::raw::c_int,
    pub alpha_filtering: ::std::os::raw::c_int,
    pub alpha_quality: ::std::os::raw::c_int,
    pub pass: ::std::os::raw::c_int,
    pub show_compressed: ::std::os::raw::c_int,
    pub preprocessing: ::std::os::raw::c_int,
    pub partitions: ::std::os::raw::c_int,
    pub partition_limit: ::std::os::raw::c_int,
    pub emulate_jpeg_size: ::std::os::raw::c_int,
    pub thread_level: ::std::os::raw::c_int,
    pub low_memory: ::std::os::raw::c_int,
    pub near_lossless: ::std::os::raw::c_int,
    pub exact: ::std::os::raw::c_int,
    pub use_delta_palette: ::std::os::raw::c_int,
    pub use_sharp_yuv: ::std::os::raw::c_int,
    pub qmin: ::std::os::raw::c_int,
    pub qmax: ::std::os::raw::c_int,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPPreset {
    WEBP_PRESET_DEFAULT = 0,
    WEBP_PRESET_PICTURE = 1,
    WEBP_PRESET_PHOTO = 2,
    WEBP_PRESET_DRAWING = 3,
    WEBP_PRESET_ICON = 4,
    WEBP_PRESET_TEXT = 5,
}
extern "C" {
    pub fn WebPConfigInitInternal(
        arg1: *mut WebPConfig,
        arg2: WebPPreset,
        arg3: f32,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPConfigLosslessPreset(
        config: *mut WebPConfig,
        level: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPValidateConfig(config: *const WebPConfig) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPAuxStats {
    pub coded_size: ::std::os::raw::c_int,
    pub PSNR: [f32; 5usize],
    pub block_count: [::std::os::raw::c_int; 3usize],
    pub header_bytes: [::std::os::raw::c_int; 2usize],
    pub residual_bytes: [[::std::os::raw::c_int; 4usize]; 3usize],
    pub segment_size: [::std::os::raw::c_int; 4usize],
    pub segment_quant: [::std::os::raw::c_int; 4usize],
    pub segment_level: [::std::os::raw::c_int; 4usize],
    pub alpha_data_size: ::std::os::raw::c_int,
    pub layer_data_size: ::std::os::raw::c_int,
    pub lossless_features: u32,
    pub histogram_bits: ::std::os::raw::c_int,
    pub transform_bits: ::std::os::raw::c_int,
    pub cache_bits: ::std::os::raw::c_int,
    pub palette_size: ::std::os::raw::c_int,
    pub lossless_size: ::std::os::raw::c_int,
    pub lossless_hdr_size: ::std::os::raw::c_int,
    pub lossless_data_size: ::std::os::raw::c_int,
    pub pad: [u32; 2usize],
}
pub type WebPWriterFunction = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const u8,
        data_size: usize,
        picture: *const WebPPicture,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPMemoryWriter {
    pub mem: *mut u8,
    pub size: usize,
    pub max_size: usize,
    pub pad: [u32; 1usize],
}
extern "C" {
    pub fn WebPMemoryWriterInit(writer: *mut WebPMemoryWriter);
}
extern "C" {
    pub fn WebPMemoryWriterClear(writer: *mut WebPMemoryWriter);
}
extern "C" {
    pub fn WebPMemoryWrite(
        data: *const u8,
        data_size: usize,
        picture: *const WebPPicture,
    ) -> ::std::os::raw::c_int;
}
pub type WebPProgressHook = ::std::option::Option<
    unsafe extern "C" fn(
        percent: ::std::os::raw::c_int,
        picture: *const WebPPicture,
    ) -> ::std::os::raw::c_int,
>;
impl WebPEncCSP {
    pub const WEBP_CSP_ALPHA_BIT: WebPEncCSP = WebPEncCSP::WEBP_YUV420A;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPEncCSP {
    WEBP_YUV420 = 0,
    WEBP_YUV420A = 4,
    WEBP_CSP_UV_MASK = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPEncodingError {
    VP8_ENC_OK = 0,
    VP8_ENC_ERROR_OUT_OF_MEMORY = 1,
    VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY = 2,
    VP8_ENC_ERROR_NULL_PARAMETER = 3,
    VP8_ENC_ERROR_INVALID_CONFIGURATION = 4,
    VP8_ENC_ERROR_BAD_DIMENSION = 5,
    VP8_ENC_ERROR_PARTITION0_OVERFLOW = 6,
    VP8_ENC_ERROR_PARTITION_OVERFLOW = 7,
    VP8_ENC_ERROR_BAD_WRITE = 8,
    VP8_ENC_ERROR_FILE_TOO_BIG = 9,
    VP8_ENC_ERROR_USER_ABORT = 10,
    VP8_ENC_ERROR_LAST = 11,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPPicture {
    ///
    pub use_argb: ::std::os::raw::c_int,
    pub colorspace: WebPEncCSP,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub y: *mut u8,
    pub u: *mut u8,
    pub v: *mut u8,
    pub y_stride: ::std::os::raw::c_int,
    pub uv_stride: ::std::os::raw::c_int,
    pub a: *mut u8,
    pub a_stride: ::std::os::raw::c_int,
    pub pad1: [u32; 2usize],
    pub argb: *mut u32,
    pub argb_stride: ::std::os::raw::c_int,
    pub pad2: [u32; 3usize],
    ///
    pub writer: WebPWriterFunction,
    pub custom_ptr: *mut ::std::os::raw::c_void,
    pub extra_info_type: ::std::os::raw::c_int,
    pub extra_info: *mut u8,
    ///
    pub stats: *mut WebPAuxStats,
    pub error_code: WebPEncodingError,
    pub progress_hook: WebPProgressHook,
    pub user_data: *mut ::std::os::raw::c_void,
    pub pad3: [u32; 3usize],
    pub pad4: *mut u8,
    pub pad5: *mut u8,
    pub pad6: [u32; 8usize],
    ///
    pub memory_: *mut ::std::os::raw::c_void,
    pub memory_argb_: *mut ::std::os::raw::c_void,
    pub pad7: [*mut ::std::os::raw::c_void; 2usize],
}
extern "C" {
    pub fn WebPPictureInitInternal(
        arg1: *mut WebPPicture,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureAlloc(picture: *mut WebPPicture) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureFree(picture: *mut WebPPicture);
}
extern "C" {
    pub fn WebPPictureCopy(src: *const WebPPicture, dst: *mut WebPPicture)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPlaneDistortion(
        src: *const u8,
        src_stride: usize,
        ref_: *const u8,
        ref_stride: usize,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        x_step: usize,
        type_: ::std::os::raw::c_int,
        distortion: *mut f32,
        result: *mut f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureDistortion(
        src: *const WebPPicture,
        ref_: *const WebPPicture,
        metric_type: ::std::os::raw::c_int,
        result: *mut f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureCrop(
        picture: *mut WebPPicture,
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureView(
        src: *const WebPPicture,
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        dst: *mut WebPPicture,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureIsView(picture: *const WebPPicture) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureRescale(
        pic: *mut WebPPicture,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureImportRGB(
        picture: *mut WebPPicture,
        rgb: *const u8,
        rgb_stride: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureImportRGBA(
        picture: *mut WebPPicture,
        rgba: *const u8,
        rgba_stride: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureImportRGBX(
        picture: *mut WebPPicture,
        rgbx: *const u8,
        rgbx_stride: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureImportBGR(
        picture: *mut WebPPicture,
        bgr: *const u8,
        bgr_stride: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureImportBGRA(
        picture: *mut WebPPicture,
        bgra: *const u8,
        bgra_stride: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureImportBGRX(
        picture: *mut WebPPicture,
        bgrx: *const u8,
        bgrx_stride: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureARGBToYUVA(
        picture: *mut WebPPicture,
        arg1: WebPEncCSP,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureARGBToYUVADithered(
        picture: *mut WebPPicture,
        colorspace: WebPEncCSP,
        dithering: f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureSharpARGBToYUVA(picture: *mut WebPPicture) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureSmartARGBToYUVA(picture: *mut WebPPicture) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPPictureYUVAToARGB(picture: *mut WebPPicture) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPCleanupTransparentArea(picture: *mut WebPPicture);
}
extern "C" {
    pub fn WebPPictureHasTransparency(picture: *const WebPPicture) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPBlendAlpha(pic: *mut WebPPicture, background_rgb: u32);
}
extern "C" {
    pub fn WebPEncode(
        config: *const WebPConfig,
        picture: *mut WebPPicture,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPIDecoder {
    _unused: [u8; 0],
}
extern "C" {
    pub fn WebPGetDecoderVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPGetInfo(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPDecodeRGBA(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeARGB(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeBGRA(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeRGB(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeBGR(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeYUV(
        data: *const u8,
        data_size: usize,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        u: *mut *mut u8,
        v: *mut *mut u8,
        stride: *mut ::std::os::raw::c_int,
        uv_stride: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeRGBAInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeARGBInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeBGRAInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeRGBInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeBGRInto(
        data: *const u8,
        data_size: usize,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPDecodeYUVInto(
        data: *const u8,
        data_size: usize,
        luma: *mut u8,
        luma_size: usize,
        luma_stride: ::std::os::raw::c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: ::std::os::raw::c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: ::std::os::raw::c_int,
    ) -> *mut u8;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WEBP_CSP_MODE {
    MODE_RGB = 0,
    MODE_RGBA = 1,
    MODE_BGR = 2,
    MODE_BGRA = 3,
    MODE_ARGB = 4,
    MODE_RGBA_4444 = 5,
    MODE_RGB_565 = 6,
    MODE_rgbA = 7,
    MODE_bgrA = 8,
    MODE_Argb = 9,
    MODE_rgbA_4444 = 10,
    MODE_YUV = 11,
    MODE_YUVA = 12,
    MODE_LAST = 13,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPRGBABuffer {
    pub rgba: *mut u8,
    pub stride: ::std::os::raw::c_int,
    pub size: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPYUVABuffer {
    pub y: *mut u8,
    pub u: *mut u8,
    pub v: *mut u8,
    pub a: *mut u8,
    pub y_stride: ::std::os::raw::c_int,
    pub u_stride: ::std::os::raw::c_int,
    pub v_stride: ::std::os::raw::c_int,
    pub a_stride: ::std::os::raw::c_int,
    pub y_size: usize,
    pub u_size: usize,
    pub v_size: usize,
    pub a_size: usize,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub is_external_memory: ::std::os::raw::c_int,
    pub u: WebPDecBuffer__bindgen_ty_1,
    pub pad: [u32; 4usize],
    pub private_memory: *mut u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WebPDecBuffer__bindgen_ty_1 {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
    _bindgen_union_align: [u64; 10usize],
}
extern "C" {
    pub fn WebPInitDecBufferInternal(
        arg1: *mut WebPDecBuffer,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPFreeDecBuffer(buffer: *mut WebPDecBuffer);
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VP8StatusCode {
    VP8_STATUS_OK = 0,
    VP8_STATUS_OUT_OF_MEMORY = 1,
    VP8_STATUS_INVALID_PARAM = 2,
    VP8_STATUS_BITSTREAM_ERROR = 3,
    VP8_STATUS_UNSUPPORTED_FEATURE = 4,
    VP8_STATUS_SUSPENDED = 5,
    VP8_STATUS_USER_ABORT = 6,
    VP8_STATUS_NOT_ENOUGH_DATA = 7,
}
extern "C" {
    pub fn WebPINewDecoder(output_buffer: *mut WebPDecBuffer) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPINewRGB(
        csp: WEBP_CSP_MODE,
        output_buffer: *mut u8,
        output_buffer_size: usize,
        output_stride: ::std::os::raw::c_int,
    ) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPINewYUVA(
        luma: *mut u8,
        luma_size: usize,
        luma_stride: ::std::os::raw::c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: ::std::os::raw::c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: ::std::os::raw::c_int,
        a: *mut u8,
        a_size: usize,
        a_stride: ::std::os::raw::c_int,
    ) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPINewYUV(
        luma: *mut u8,
        luma_size: usize,
        luma_stride: ::std::os::raw::c_int,
        u: *mut u8,
        u_size: usize,
        u_stride: ::std::os::raw::c_int,
        v: *mut u8,
        v_size: usize,
        v_stride: ::std::os::raw::c_int,
    ) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPIDelete(idec: *mut WebPIDecoder);
}
extern "C" {
    pub fn WebPIAppend(idec: *mut WebPIDecoder, data: *const u8, data_size: usize)
        -> VP8StatusCode;
}
extern "C" {
    pub fn WebPIUpdate(idec: *mut WebPIDecoder, data: *const u8, data_size: usize)
        -> VP8StatusCode;
}
extern "C" {
    pub fn WebPIDecGetRGB(
        idec: *const WebPIDecoder,
        last_y: *mut ::std::os::raw::c_int,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        stride: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPIDecGetYUVA(
        idec: *const WebPIDecoder,
        last_y: *mut ::std::os::raw::c_int,
        u: *mut *mut u8,
        v: *mut *mut u8,
        a: *mut *mut u8,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        stride: *mut ::std::os::raw::c_int,
        uv_stride: *mut ::std::os::raw::c_int,
        a_stride: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn WebPIDecodedArea(
        idec: *const WebPIDecoder,
        left: *mut ::std::os::raw::c_int,
        top: *mut ::std::os::raw::c_int,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> *const WebPDecBuffer;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPBitstreamFeatures {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub has_alpha: ::std::os::raw::c_int,
    pub has_animation: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
    pub pad: [u32; 5usize],
}
extern "C" {
    pub fn WebPGetFeaturesInternal(
        arg1: *const u8,
        arg2: usize,
        arg3: *mut WebPBitstreamFeatures,
        arg4: ::std::os::raw::c_int,
    ) -> VP8StatusCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPDecoderOptions {
    pub bypass_filtering: ::std::os::raw::c_int,
    pub no_fancy_upsampling: ::std::os::raw::c_int,
    pub use_cropping: ::std::os::raw::c_int,
    pub crop_left: ::std::os::raw::c_int,
    pub crop_top: ::std::os::raw::c_int,
    pub crop_width: ::std::os::raw::c_int,
    pub crop_height: ::std::os::raw::c_int,
    pub use_scaling: ::std::os::raw::c_int,
    pub scaled_width: ::std::os::raw::c_int,
    pub scaled_height: ::std::os::raw::c_int,
    pub use_threads: ::std::os::raw::c_int,
    pub dithering_strength: ::std::os::raw::c_int,
    pub flip: ::std::os::raw::c_int,
    pub alpha_dithering_strength: ::std::os::raw::c_int,
    pub pad: [u32; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WebPDecoderConfig {
    pub input: WebPBitstreamFeatures,
    pub output: WebPDecBuffer,
    pub options: WebPDecoderOptions,
}
extern "C" {
    pub fn WebPInitDecoderConfigInternal(
        arg1: *mut WebPDecoderConfig,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPIDecode(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> *mut WebPIDecoder;
}
extern "C" {
    pub fn WebPDecode(
        data: *const u8,
        data_size: usize,
        config: *mut WebPDecoderConfig,
    ) -> VP8StatusCode;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPFeatureFlags {
    ANIMATION_FLAG = 2,
    XMP_FLAG = 4,
    EXIF_FLAG = 8,
    ALPHA_FLAG = 16,
    ICCP_FLAG = 32,
    ALL_VALID_FLAGS = 62,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPMuxAnimDispose {
    WEBP_MUX_DISPOSE_NONE = 0,
    WEBP_MUX_DISPOSE_BACKGROUND = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPMuxAnimBlend {
    WEBP_MUX_BLEND = 0,
    WEBP_MUX_NO_BLEND = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPData {
    pub bytes: *const u8,
    pub size: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPMux {
    _unused: [u8; 0],
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPMuxError {
    WEBP_MUX_OK = 1,
    WEBP_MUX_NOT_FOUND = 0,
    WEBP_MUX_INVALID_ARGUMENT = -1,
    WEBP_MUX_BAD_DATA = -2,
    WEBP_MUX_MEMORY_ERROR = -3,
    WEBP_MUX_NOT_ENOUGH_DATA = -4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WebPChunkId {
    WEBP_CHUNK_VP8X = 0,
    WEBP_CHUNK_ICCP = 1,
    WEBP_CHUNK_ANIM = 2,
    WEBP_CHUNK_ANMF = 3,
    WEBP_CHUNK_DEPRECATED = 4,
    WEBP_CHUNK_ALPHA = 5,
    WEBP_CHUNK_IMAGE = 6,
    WEBP_CHUNK_EXIF = 7,
    WEBP_CHUNK_XMP = 8,
    WEBP_CHUNK_UNKNOWN = 9,
    WEBP_CHUNK_NIL = 10,
}
extern "C" {
    pub fn WebPGetMuxVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPNewInternal(arg1: ::std::os::raw::c_int) -> *mut WebPMux;
}
extern "C" {
    pub fn WebPMuxDelete(mux: *mut WebPMux);
}
extern "C" {
    pub fn WebPMuxCreateInternal(
        arg1: *const WebPData,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> *mut WebPMux;
}
extern "C" {
    pub fn WebPMuxSetChunk(
        mux: *mut WebPMux,
        fourcc: *const ::std::os::raw::c_char,
        chunk_data: *const WebPData,
        copy_data: ::std::os::raw::c_int,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxGetChunk(
        mux: *const WebPMux,
        fourcc: *const ::std::os::raw::c_char,
        chunk_data: *mut WebPData,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxDeleteChunk(
        mux: *mut WebPMux,
        fourcc: *const ::std::os::raw::c_char,
    ) -> WebPMuxError;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPMuxFrameInfo {
    pub bitstream: WebPData,
    pub x_offset: ::std::os::raw::c_int,
    pub y_offset: ::std::os::raw::c_int,
    pub duration: ::std::os::raw::c_int,
    pub id: WebPChunkId,
    pub dispose_method: WebPMuxAnimDispose,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [u32; 1usize],
}
extern "C" {
    pub fn WebPMuxSetImage(
        mux: *mut WebPMux,
        bitstream: *const WebPData,
        copy_data: ::std::os::raw::c_int,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxPushFrame(
        mux: *mut WebPMux,
        frame: *const WebPMuxFrameInfo,
        copy_data: ::std::os::raw::c_int,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxGetFrame(
        mux: *const WebPMux,
        nth: u32,
        frame: *mut WebPMuxFrameInfo,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxDeleteFrame(mux: *mut WebPMux, nth: u32) -> WebPMuxError;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPMuxAnimParams {
    pub bgcolor: u32,
    pub loop_count: ::std::os::raw::c_int,
}
extern "C" {
    pub fn WebPMuxSetAnimationParams(
        mux: *mut WebPMux,
        params: *const WebPMuxAnimParams,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxGetAnimationParams(
        mux: *const WebPMux,
        params: *mut WebPMuxAnimParams,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxSetCanvasSize(
        mux: *mut WebPMux,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxGetCanvasSize(
        mux: *const WebPMux,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxGetFeatures(mux: *const WebPMux, flags: *mut u32) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxNumChunks(
        mux: *const WebPMux,
        id: WebPChunkId,
        num_elements: *mut ::std::os::raw::c_int,
    ) -> WebPMuxError;
}
extern "C" {
    pub fn WebPMuxAssemble(mux: *mut WebPMux, assembled_data: *mut WebPData) -> WebPMuxError;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPAnimEncoder {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WebPAnimEncoderOptions {
    pub anim_params: WebPMuxAnimParams,
    pub minimize_size: ::std::os::raw::c_int,
    pub kmin: ::std::os::raw::c_int,
    pub kmax: ::std::os::raw::c_int,
    pub allow_mixed: ::std::os::raw::c_int,
    pub verbose: ::std::os::raw::c_int,
    pub padding: [u32; 4usize],
}
extern "C" {
    pub fn WebPAnimEncoderOptionsInitInternal(
        arg1: *mut WebPAnimEncoderOptions,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPAnimEncoderNewInternal(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *const WebPAnimEncoderOptions,
        arg4: ::std::os::raw::c_int,
    ) -> *mut WebPAnimEncoder;
}
extern "C" {
    pub fn WebPAnimEncoderAdd(
        enc: *mut WebPAnimEncoder,
        frame: *mut WebPPicture,
        timestamp_ms: ::std::os::raw::c_int,
        config: *const WebPConfig,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPAnimEncoderAssemble(
        enc: *mut WebPAnimEncoder,
        webp_data: *mut WebPData,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn WebPAnimEncoderGetError(enc: *mut WebPAnimEncoder) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn WebPAnimEncoderDelete(enc: *mut WebPAnimEncoder);
}
