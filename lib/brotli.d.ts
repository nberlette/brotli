// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file

/**
 * Decompresses a Brotli-compressed buffer, returning the decompressed data.
 *
 * If the data cannot be decompressed, a copy of the input is returned.
 *
 * @param {Uint8Array} data - The Brotli-compressed data to decompress.
 * @returns {Uint8Array} - The decompressed data.
 */
export function decompress(data: Uint8Array): Uint8Array;
