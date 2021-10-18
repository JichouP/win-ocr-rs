fn main() {
  windows::build! {
    Windows::Graphics::Imaging::{BitmapDecoder, SoftwareBitmap},
    Windows::Media::Ocr::{OcrEngine, OcrResult},
    Windows::Storage::{FileAccessMode, StorageFile},
  }
}
