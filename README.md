# libwebp-sys

[bindgen](https://github.com/servo/rust-bindgen)'d FFI bindings to [libwebp](https://developers.google.com/speed/webp/docs/api).

libwebp is built with the [`cc`](//lib.rs/cc) crate. It needs a C compiler, but `cmake` is not used.

Set `RUSTFLAGS="-Ctarget-cpu=native"` or your desired CPU architecture to optimize the C code for it.

## Usage

Add the following to the `Cargo.toml` in your project:

```toml
[dependencies]
libwebp-sys = "0.9"
```

or to require `no_std` support:

```toml
libwebp-sys = { version = "0.9", default-features = false, features = ["parallel"] }
```

The `neon`, `sse41` and `avx2` feature flags can be set to force support for Neon, SSE 4.1 and AVX2
respectively, but this is usually unnecessary as it can be set through
`-Ctarget-feature` (e.g. `RUSTFLAGS="-Ctarget-feature=avx2"`) as well.

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
