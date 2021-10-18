use std::fs;
use win_ocr_bindings::Windows::{
    Globalization::Language,
    Graphics::Imaging::{BitmapDecoder, SoftwareBitmap},
    Media::Ocr::OcrEngine,
    Storage::{FileAccessMode, StorageFile},
};

/// Performs OCR on PNG files and returns the result.
///
/// Language will be automatically determined based on the environment.
///
/// If you want to choose your own language, use `ocr_with_lang`.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the image
///
/// # Example
///
/// ```
/// use win_ocr::ocr;
/// let ocr_result: String = ocr("/path/to/file.png");
/// ```
pub fn ocr(path: &str) -> windows::Result<String> {
    Ok(ocr_from_bitmap(open_image_as_bitmap(path)?)?)
}

/// Performs OCR on PNG files and returns the result.
///
/// If you want to use the default language of your environment, use `ocr`.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the image
/// * `lang` - A string slice that holds the language to use
///
/// # Example
///
/// ```
/// use win_ocr::ocr_with_lang;
/// let ocr_result: String = ocr_with_lang("/path/to/file.png", "en");
/// ```
pub fn ocr_with_lang(path: &str, lang: &str) -> windows::Result<String> {
    Ok(ocr_from_bitmap_with_lang(
        open_image_as_bitmap(path)?,
        lang,
    )?)
}

/// Performs OCR on `SoftwareBitmap` and returns the result.
///
/// Please consider using `ocr` instead.
pub fn ocr_from_bitmap(bitmap: SoftwareBitmap) -> windows::Result<String> {
    let lang = &OcrEngine::AvailableRecognizerLanguages()?
        .First()?
        .Current()?
        .LanguageTag()?;
    let lang = Language::CreateLanguage(lang)?;
    let engine = OcrEngine::TryCreateFromLanguage(lang)?;

    let result = engine
        .RecognizeAsync(bitmap)?
        .get()?
        .Text()?
        .to_string_lossy();

    Ok(result)
}

/// Performs OCR on `SoftwareBitmap` and returns the result.
///
/// Please consider using `ocr_with_lang` instead.
pub fn ocr_from_bitmap_with_lang(bitmap: SoftwareBitmap, lang: &str) -> windows::Result<String> {
    let lang = Language::CreateLanguage(lang)?;
    let engine = OcrEngine::TryCreateFromLanguage(lang)?;

    let result = engine
        .RecognizeAsync(bitmap)?
        .get()?
        .Text()?
        .to_string_lossy();

    Ok(result)
}

/// Opens an PNG file as a `SoftwareBitmap`
pub fn open_image_as_bitmap(path: &str) -> windows::Result<SoftwareBitmap> {
    let path: String = fs::canonicalize(path)
        .unwrap()
        .to_string_lossy()
        .replace("\\\\?\\", "");

    let file = StorageFile::GetFileFromPathAsync(path)?.get()?;

    let bitmap = BitmapDecoder::CreateWithIdAsync(
        BitmapDecoder::PngDecoderId()?,
        file.OpenAsync(FileAccessMode::Read)?.get()?,
    )?
    .get()?;

    bitmap.GetSoftwareBitmapAsync()?.get()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ocr_works() {
        let ocr_text: String = ocr("sample/sample.png").unwrap();
        assert_eq!(ocr_text, "Sample Text");
    }
}
