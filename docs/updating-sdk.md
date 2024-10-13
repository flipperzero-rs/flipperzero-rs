# Updating the supported SDK

The [`flipperzero-sys`] crate now generates bindings at compile time using a [`build.rs`] script.

By default this will fetch the currently supported SDK and toolchain; then use those to generate
the required Rust bindings.

The selected SDK and Toolchain versions can be explicitly specified using the following environment variables:

- `FLIPPER_SDK_VERSION`
- `FLIPPER_TOOLCHAIN_VERSION`

The default values can be found in the `flipperzero-sys` [`build.rs`] script.
Make sure to update the SDK details in [`README.md`] before making a new release.

[`build.rs`]: ../crates/sys/build.rs
[`flipperzero-sys`]: https://crates.io/crates/flipperzero-sys
[`README.md`]: ../README.md
