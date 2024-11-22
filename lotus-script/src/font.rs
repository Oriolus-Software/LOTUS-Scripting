use lotus_script_sys::FfiObject;
use lotus_shared::content::ContentId;
pub use lotus_shared::font::*;

/// Get the properties of a bitmap font.
/// Returns `None` if the font is not currently loaded. It will be loaded in the background.
/// Just call this function again later until it returns `Some`.
pub fn bitmap_font_properties(font: ContentId) -> Option<BitmapFontProperties> {
    let font = FfiObject::new(&font);
    let properties = unsafe { lotus_script_sys::font::bitmap_font_properties(font.packed()) };

    if properties == 0 {
        None
    } else {
        Some(FfiObject::from_packed(properties).deserialize())
    }
}

/// Get the width of the text in pixels.
/// Returns `None` if the font is not currently loaded. It will be loaded in the background.
/// Just call this function again later until it returns `Some`.
pub fn text_len(font: ContentId, text: &str, letter_spacing: i32) -> Option<u32> {
    let font = FfiObject::new(&font);
    let text = FfiObject::new(&text);

    let len =
        unsafe { lotus_script_sys::font::text_len(font.packed(), text.packed(), letter_spacing) };

    if len < 0 {
        None
    } else {
        Some(len as u32)
    }
}
