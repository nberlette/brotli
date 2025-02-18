#![allow(unexpected_cfgs)]

use wasm_bindgen::prelude::*;

/// Decompresses a Brotli-compressed buffer, returning the decompressed data.
///
/// If the data cannot be decompressed, a copy of the input is returned.
///
/// @param {Uint8Array} data - The Brotli-compressed data to decompress.
/// @returns {Uint8Array} - The decompressed data.
#[wasm_bindgen(js_name = decompress, skip_jsdoc)]
pub fn decompress_js(data: Box<[u8]>) -> Box<[u8]> {
  decompress(data)
}

/// Decompresses a Brotli-compressed buffer, returning the decompressed data.
///
/// If the data cannot be decompressed, a copy of the input is returned.
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
