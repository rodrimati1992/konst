[![Rust](https://github.com/rodrimati1992/konst_crates/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/konst_crates/actions)
[![crates-io](https://img.shields.io/crates/v/konst.svg)](https://crates.io/crates/konst)
[![api-docs](https://docs.rs/konst/badge.svg)](https://docs.rs/konst/*)

Compile-time comparison, parsing, and const equivalents of std methods.


# No-std support

`konst` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`konst` requires Rust 1.46.0, because it uses looping an branching in const contexts.

Features that require newer versions of Rust, or the nightly compiler,
need to be explicitly enabled with cargo features.
