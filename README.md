# Pathlib

[![codecov](https://codecov.io/gh/TheVeryDarkness/pathlib/graph/badge.svg?token=ESDSZCI3Y5)](https://codecov.io/gh/TheVeryDarkness/pathlib)

A module that provides types representing filesystem paths with semantics appropriate for different operating systems.

Unlike `std::path::Path`, `Pathlib` is a full-featured path manipulation library that provides a high-level API for path operations.

## Motivation

I found that though both Rust and WASM are portable, WASM compiled with **wasm32-unknown-emscripten** is not actually portable.

- Without `-lnoderawfs.js`, emscripten provides a virtual POSIX-style filesystem, which may differ from the host filesystem. However, Rust's `std::path::Path` is designed to work with the host filesystem.
- With `-lnoderawfs.js`, emscripten provides direct access to the host filesystem, but it is not portable across different platforms. You can't run the same WASM binary compiled from Rust on both Windows and Linux.
