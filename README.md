# Rust for Flipper Zero 🐬❤️🦀

[![crates.io](https://img.shields.io/crates/v/flipperzero)](https://crates.io/crates/flipperzero)
[![Flipper Zero API](https://img.shields.io/badge/Flipper%20Zero%20API-87.1-orange)](https://github.com/flipperdevices/flipperzero-firmware/blob/1.4.3/targets/f7/api_symbols.csv)
[![docs.rs](https://img.shields.io/docsrs/flipperzero)](https://docs.rs/flipperzero)
[![MIT license](https://img.shields.io/crates/l/flipperzero)](LICENSE)

This project allows writing Rust-based applications for the [Flipper Zero](https://flipperzero.one/).

It doesn't have any direct dependency on [`flipperzero-firmware`](https://github.com/flipperdevices/flipperzero-firmware) or toolchain,
so it can be used to build binaries with no external dependencies.

These crates only support the [`core`](https://doc.rust-lang.org/core/) and [`alloc`](https://doc.rust-lang.org/alloc/) crates.

The Rust `thumbv7em-none-eabihf` target currently only supports [`no_std`](https://rust-embedded.github.io/book/intro/no-std.html) development.
This means it's not possible to use anything in the [`std`](https://doc.rust-lang.org/std/) crate.

## SDK version

Currently supports SDK 87.1 ([flipperzero-firmware@1.3.4](https://github.com/flipperdevices/flipperzero-firmware/tree/1.3.4)).

The crate major version number will be updated after a bump in [API version](https://github.com/flipperdevices/flipperzero-firmware/blob/release/targets/f7/api_symbols.csv) in the Flipper Zero firmware.

| Crate version | API version |
| ------------- | ----------- |
| HEAD          | 87.1        |
| 0.16.x        | 87.1        |
| 0.15.x        | 86.0        |
| 0.14.x        | 79.2        |
| 0.13.x        | 78.0        |
| 0.12.x        | 73.0        |
| 0.11.x        | 35.0        |
| 0.10.x        | 28.2        |
| 0.9.x         | 23.0        |
| 0.8.x         | 20.0        |
| 0.7.x         | 14.0        |
| 0.6.x         | 11.2        |
| 0.5.x         | 10.1        |
| 0.4.x         | 7.5         |
| 0.3.x         | 2.2         |

## Crates

- [`flipperzero`](https://crates.io/crates/flipperzero): High-level safe bindings
- [`flipperzero-alloc`](https://crates.io/crates/flipperzero-alloc): Custom [global allocator](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) (required for [`alloc`](https://doc.rust-lang.org/alloc/))
- [`flipperzero-rt`](https://crates.io/crates/flipperzero-rt): Runtime support (including [panic handler](https://docs.rs/flipperzero-rt/latest/flipperzero_rt/panic_handler/) and [entry point](https://docs.rs/flipperzero-rt/latest/flipperzero_rt/macro.entry.html) helper)
- [`flipperzero-sys`](https://crates.io/crates/flipperzero-sys): Low-level bindings to Flipper Zero API (unsafe)

## Initial setup

1. Install [`rustup`](https://rust-lang.github.io/rustup/) by following the instructions on [`rustup.rs`](https://rustup.rs/).
2. Use `rustup` to install the `thumbv7em-none-eabihf` target:

   ```
   rustup target add thumbv7em-none-eabihf
   ```

## Writing an application

The Flipper Zero supports installing [externally built applications on the SD card](https://github.com/flipperdevices/flipperzero-firmware/blob/dev/documentation/AppsOnSDCard.md).

See [`flipperzero-template`](https://github.com/flipperzero-rs/flipperzero-template)🚀 to help you get started.

## License

Licensed under the MIT License. See LICENSE for details.
