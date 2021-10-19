# win-ocr

[![crate-name at crates.io](https://img.shields.io/crates/v/win_ocr.svg)](https://crates.io/crates/win_ocr) [![crate-name at docs.rs](https://docs.rs/win_ocr/badge.svg)](https://docs.rs/win_ocr)

A library for Rust to do OCR on Windows.

## Usage

```rust
use win_ocr::ocr;

let ocr_text: String = ocr("/path/to/file.png").unwrap();
assert_eq!(ocr_text, "OCR Text");
```
