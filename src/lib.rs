use std::fs;
use windows::{
    core::HSTRING,
    Globalization::Language,
    Graphics::Imaging::{BitmapDecoder, SoftwareBitmap},
    Media::Ocr::OcrEngine,
    Storage::{FileAccessMode, StorageFile},
};

use windows::core::{Error, Result, HRESULT};

const E_ACCESSDENIED: HRESULT = HRESULT(0x80070005u32 as i32);

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
/// let ocr_result = ocr("/path/to/file.png");
/// ```
pub fn ocr(path: &str) -> Result<String> {
    let bitmap = open_image_as_bitmap(path)?;
    let ocr_result = ocr_from_bitmap(bitmap)?;
    Ok(ocr_result)
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
/// let ocr_result = ocr_with_lang("/path/to/file.png", "en");
/// ```
pub fn ocr_with_lang(path: &str, lang: &str) -> Result<String> {
    let bitmap = open_image_as_bitmap(path)?;
    let ocr_result = ocr_from_bitmap_with_lang(bitmap, lang)?;
    Ok(ocr_result)
}

/// Performs OCR on `SoftwareBitmap` and returns the result.
///
/// Please consider using `ocr` instead.
pub fn ocr_from_bitmap(bitmap: SoftwareBitmap) -> Result<String> {
    let lang = &OcrEngine::AvailableRecognizerLanguages()?
        .First()?
        .Current()?
        .LanguageTag()?;

    let lang = Language::CreateLanguage(lang)?;
    let engine = OcrEngine::TryCreateFromLanguage(&lang)?;

    let result = engine
        .RecognizeAsync(&bitmap)?
        .get()?
        .Text()?
        .to_string_lossy();

    Ok(result)
}

/// Performs OCR on `SoftwareBitmap` and returns the result.
///
/// Please consider using `ocr_with_lang` instead.
pub fn ocr_from_bitmap_with_lang(bitmap: SoftwareBitmap, lang: &str) -> Result<String> {
    let lang = Language::CreateLanguage(&HSTRING::from(lang))?;
    let engine = OcrEngine::TryCreateFromLanguage(&lang)?;

    let result = engine
        .RecognizeAsync(&bitmap)?
        .get()?
        .Text()?
        .to_string_lossy();

    Ok(result)
}

/// Opens an PNG file as a `SoftwareBitmap`
pub fn open_image_as_bitmap(path: &str) -> Result<SoftwareBitmap> {
    let path = fs::canonicalize(path);
    let path = match path {
        Ok(path) => path.to_string_lossy().replace("\\\\?\\", ""),
        Err(_) => {
            return Err(Error::new(E_ACCESSDENIED, "Could not open file".into()));
        }
    };

    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path))?.get()?;

    let bitmap = BitmapDecoder::CreateWithIdAsync(
        BitmapDecoder::PngDecoderId()?,
        &file.OpenAsync(FileAccessMode::Read)?.get()?,
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

        let ocr_text: String = ocr("sample/sample_ja.png").unwrap().replace(' ', "");
        assert_eq!(ocr_text, "サンプルテキスト");
    }
}
