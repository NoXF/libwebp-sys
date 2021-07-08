# libwebp-sys

[bindgen](https://github.com/servo/rust-bindgen)'d FFI bindings to [libwebp](https://developers.google.com/speed/webp/docs/api).

libwebp is built with the [`cc`](//lib.rs/cc) crate. It needs a C compiler, but `cmake` is not used.

Set `TARGET_CPU` env var to `native` or your desired CPU architecture to optimize the C code for it.

## Usage

Add the following to the `Cargo.toml` in your project:

```toml
[dependencies]
libwebp-sys = "0.4"
```

or to require newer CPUs with SIMD support:

```toml
[dependencies]
libwebp-sys = { version = "0.4", features = ["avx2", "sse41", "neon"] }
```

## Examples

### Encode

```rust
pub fn encode_webp(input_image: &[u8], width: u32, height: u32, quality: i32) -> Result<Vec<u8>> {
    unsafe {
	    let mut out_buf = std::ptr::null_mut();
	    let stride = width as i32 * 4;
	    let len = WebPEncodeRGBA(input_image.as_ptr(), width as i32, height as i32, stride, quality as f32, &mut out_buf);
	    Ok(std::slice::from_raw_parts(out_buf, len as usize).into())
    }
}
```

### Decode

```rust
pub fn decode_webp(buf: &[u8]) -> Result<Vec<u8>> {
	let mut width = 0;
	let mut height = 0;
	let len = buf.len();
	unsafe {
		WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height);
		let out_buf = WebPDecodeRGBA(buf.as_ptr(), len, &mut width, &mut height);
	}
	Ok(std::slice::::from_raw_parts(out_buf, width * height * 4).into())
}
```
