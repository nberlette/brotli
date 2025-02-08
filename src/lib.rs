//! # brocha
//!
//! This crate provides a fast, lightweight implementation of Google's Brotli
//! decompression algorithm in WebAssembly.
//!
//! Under the hood this is simply a wrapper of the `brotli-decompressor` crate,
//! with WebAssembly bindings that make it suitable for use in Deno, Node, Bun,
//! and other modern JS runtimes (including browsers and Cloudflare Workers).
//!
//! Currently `brocha` is not published to crates.io, as its primary purpose in
//! life is to be a WebAssembly alternative to brocha's primary decompression
//! implementation, which is written in pure TypeScript.
//!
//! Why include a WebAssembly implementation **and** a TypeScript one as well?
//! Despite serving the same end goal, the two implementations are designed to
//! serve different use cases. The TypeScript implementation is designed to be
//! compatible with **any** ES2015+ environment right out of the box, with no
//! dependencies and no instantiation or configuration required. Despite being
//! written in TypeScript, the performance of the main `brocha` implementation
//! is quite impressive in its own right, and is more than sufficient for the
//! vast majority of use cases.
//!
//! Sometimes, however, you need to squeeze every last bit of performance out
//! of the decompression process. This is where the WebAssembly implementation
//! comes in, but it comes with a few caveats:
//!
//! 1. The runtime environment must support WebAssembly, obviously.
//! 2. The WebAssembly module must be loaded and instantiated before use.
//! 3. The WebAssembly build is about 30% larger than the TypeScript build.
//!
//! @see https://jsr.io/@nick/brocha/doc for the JSR documentation.
//! @see https://npmjs.com/package/brocha for the brocha NPM distribution.
//! @see https://github.com/nberlette/brocha#readme for the GitHub repository.
#![allow(unexpected_cfgs)]

use wasm_bindgen::prelude::*;

/// Decompresses a
#[wasm_bindgen(js_name = decompress)]
pub fn decompress_js(data: Box<[u8]>) -> Box<[u8]> {
  decompress(data)
}

pub fn decompress(data: impl AsRef<[u8]>) -> Box<[u8]> {
  let mut output = Vec::new();

  match ::brotli_decompressor::BrotliDecompress(&mut data.as_ref(), &mut output)
  {
    Ok(_) => output.into(),
    Err(_) => (*data.as_ref()).into(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_decompress_empty_buffer() {
    // magic number for empty brotli stream
    let data: Box<[u8]> = vec![0x06].into();
    let result = decompress(data);
    assert_eq!(result.len(), 0, "Decompressed data should be empty");
  }

  #[test]
  fn test_decompress_hello_world() {
    let data = vec![
      139, 5, 128, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 3,
    ];
    let result = decompress(data);
    assert_eq!(
      result,
      b"Hello, World".as_slice().into(),
      "Decompressed data should equal 'Hello, World'"
    );
  }

  #[test]
  fn test_decompress_uncompressed_data_returned_as_is() {
    let data = vec![0x00];
    let result = decompress(data);
    assert_eq!(
      result,
      vec![0x00].into_boxed_slice(),
      "Invalid magic number should return input data"
    );
  }

  #[test]
  fn test_decompress_invalid_data_returned_as_is() {
    let data = vec![0x01, 0x02, 0x03];
    let result = decompress(data);
    assert_eq!(
      result,
      vec![0x01, 0x02, 0x03].into_boxed_slice(),
      "Invalid input data should be returned as is"
    );
  }

  #[test]
  fn test_decompress_valid_size_hint_with_invalid_data() {
    let data = vec![0x06, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    let result = decompress(data);
    assert_eq!(
      result,
      vec![].into_boxed_slice(),
      "Invalid input data should be returned as is"
    );
  }
}
