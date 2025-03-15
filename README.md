<div align="center">

# [`@nick/brotli`][JSR]

<big><b>Lightweight [brotli] decompressor, powered by WebAssembly.</b></big>

<small><b>Supported by Deno, Bun, Node, modern browsers, and more.</b></small>

[![][badge-jsr-score]][badge-jsr-score] [![][badge-jsr-pkg]][badge-jsr-pkg]

</div>

---

This module provides a blazing-fast [brotli] decompressor, written in Rust and
compiled to WebAssembly. It's compatibile with virtually any runtime supportive
of WebAssembly, including [Deno], [Bun], [Node], [Cloudflare Workers], and all
recent releases of modern browsers (Chrome, Firefox, Safari, Edge, etc.).

## Install

```sh
deno add jsr:@nick/brotli
```

```sh
npx jsr add @nick/brotli
```

```sh
bunx jsr add @nick/brotli
```

```sh
pnpm dlx jsr add @nick/brotli
```

```sh
yarn dlx jsr add @nick/brotli
```

---

## Usage

```ts
import { decompress } from "@nick/brotli";

const res = await fetch("file:///compressed.txt.br");
const small = await res.bytes();

const large = decompress(small); // <- synchronous

console.log(`Decompressed ${small.length}B -> ${large.length}B`);
```

> [!IMPORTANT]
>
> The `decompress` function is pre-initialized and ready for immediate use.

---

## Overview

Under the hood on the Rust side of the codebase, this is a thin wrapper of the
third-party [brotli-decompressor] crate, adding WebAssembly bindings and some
graceful error handling[^1].

On the JavaScript side of the codebase, it adds support for several additional
input types, allowing you to decompress data from a string, `ArrayBuffer`, or
any `ArrayBufferView` object, including `DataView` and all typed array types.
This results in a **very** similar signature to that of `brotliDecompressSync`
from the native `node:zlib` module, giving you the ability to use this module as
an almost-drop-in replacement for it in _most_ situations.

> [!NOTE]
>
> This tool does not use the Node `Buffer` API, which is from a time when the
> JavaScript ecosystem didn't have a standardized way to represent binary data.
> Instead we use its superclass - the universally-supported `Uint8Array` type.

[^1]: The error handling is a bit more graceful than the original crate, but it
    might not be what you're looking for: in the case of erroneous input data
    (or data that is not compressed with Brotli), the `decompress` function will
    simply return the same input data (but always as a `Uint8Array`).

---

### Prior Art

- [brotli-wasm]
- [brotli-dec-wasm]
- [brotli-decompressor]

> [!TIP]
>
> **Shameless Plug**: If you're looking for a non-WebAssembly Brotli
> decompressor, I've also published [brocha], a 100% JS decompressor that weighs
> in at under 200KB. While it's not quite as fast as WASM, it benchmarks
> extremely well in comparison to other JS-based solutions.

---

<div align="center">

<big><strong>[MIT] © [Nicholas Berlette]. All rights reserved.</strong></big>

<small><strong>[GitHub] • [Issues] • [JSR] • [NPM] • [brocha]</strong></small>

<br>

[![JSR][badge-jsr]][JSR] [![JSR][badge-jsr-score]][JSR] [![NPM][badge-npm]][NPM]

</div>

[brocha]: https://jsr.io/@nick/brocha "View the @nick/brocha project on jsr.io"
[brotli]: https://github.com/google/brotli "View the google/brotli project on GitHub"
[MIT]: https://nick.mit-license.org "MIT © 2024+ Nicholas Berlette. All rights reserved."
[Nicholas Berlette]: https://github.com/nberlette "Nicholas Berlette on GitHub"
[GitHub]: https://github.com/nberlette/brotli "View the @nick/brotli project on GitHub"
[Issues]: https://github.com/nberlette/brotli/issues "View issues for the @nick/brotli project on GitHub"
[JSR]: https://jsr.io/@nick/brotli/doc "View the @nick/brotli documentation on jsr.io"
[NPM]: https://www.npmjs.com/package/debrotli "View the @nberlette/brotli package on npm"
[brotli-decompressor]: https://crates.io/crates/brotli-decompressor "View the brotli-decompressor crate on crates.io"
[brotli-wasm]: https://crates.io/brotli-wasm "View the brotli-wasm crate on crates.io"
[brotli-dec-wasm]: https://crates.io/brotli-dec-wasm "View the brotli-dec-wasm crate on crates.io"
[badge-npm]: https://img.shields.io/badge/debrotli-tomato.svg?logoWidth=32&logoColor=white&color=firebrick&logo=data:image/svg+xml;charset=utf-8;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzLjExZW0iIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIC0yNSA1MTIgMTkwIj48cGF0aCBmaWxsPSIjZmZmIiBkPSJNMTU3LjUzOCAxNjQuMTAzaDY1LjY0MXYtMzIuODJoNjUuNjQyVjBIMTU3LjUzOHpNMjIzLjE4IDMyLjgySDI1NnY2NS42NGgtMzIuODJ6TTMxNS4wNzcgMHYxMzEuMjgyaDY1LjY0VjMyLjgyMWgzMi44MjF2OTguNDYxaDMyLjgyMVYzMi44MjFoMzIuODJ2OTguNDYxSDUxMlYwek0wIDEzMS4yODJoNjUuNjQxVjMyLjgyMWgzMi44MnY5OC40NjFoMzIuODIxVjBIMHoiLz48L3N2Zz4= "View @nberlette/brotli on npm"
[badge-jsr]: https://jsr.io/badges/@nick "View all of @nick's packages on jsr.io"
[badge-jsr-pkg]: https://jsr.io/badges/@nick/brotli "View @nick/brotli on jsr.io"
[badge-jsr-score]: https://jsr.io/badges/@nick/brotli/score "View the score for @nick/brotli on jsr.io"
[Deno]: https://deno.land "Deno - A modern JavaScript and TypeScript runtime"
[Bun]: https://bun.sh "Bun - A fast all-in-one JavaScript runtime"
[Node]: https://nodejs.org "Node.js - A JavaScript runtime built on Chrome's V8 JavaScript engine"
[Cloudflare Workers]: https://workers.cloudflare.com "Cloudflare Workers serverless execution environment"
