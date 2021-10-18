# win-ocr-rs

A library for Rust to do OCR on Windows.

## Usage

```rust
use win_ocr::ocr;

let ocr_text: String = ocr("/path/to/file.png").unwrap();
assert_eq!(ocr_text, "OCR Text");
```
