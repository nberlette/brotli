/**
 * This module provides a pre-instantiated WebAssembly-based implementation of
 * the Brotli decompression algorithm, originally developed by Google for the
 * WOFF2 font format's compression.
 *
 * The decompressor is implemented in Rust, compiled to WebAssembly, and
 * wrapped in a dead-simple TypeScript API. It's designed to be as small and
 * fast as possible, with a minimal API surface and no external dependencies.
 *
 * **Note**: if the decompressor receives an invalid or uncompressed input, it
 * will quietly return the input buffer as-is. This is a feature, not a bug.
 *
 * @example
 * ```ts
 * import { decompress } from "@nick/brotli";
 *
 * const compressed = new Uint8Array(
 *   [0x1b, 0x5e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
 * );
 * const decompressed = decompress(compressed);
 *
 * console.log(decompressed);
 * // Uint8Array [ 27, 94, 0, 0, 0, 0, 0, 0 ]
 * ```
 * @module brotli
 */
// @ts-types="./lib/brocha.d.ts"
import * as brocha from "./lib/brocha.js";

/**
 * Decodes a brotli-compressed buffer, returning the decompressed output.
 * If decompression fails, an error will be thrown.
 *
 * @example
 * ```js
 * import { decompress } from "@nick/brotli";
 *
 * const res = await fetch("file:///compressed.wasm.br");
 * const small = new Uint8Array(await res.arrayBuffer());
 * const large = decompress(small);
 *
 * console.log(
 *   `Decompressed ${small.length}B -> ${large.length}B (${delta}%)`,
 * );
 * // Example Output:
 * //   Decompressed 1024B -> 4096B (400%)
 * ```
 * @param input The brotli-compressed buffer to decompress.
 * @returns The decompressed output.
 */
export function decompress(input: Uint8Array): Uint8Array {
  return brocha.decompress(input);
}

export default decompress;
