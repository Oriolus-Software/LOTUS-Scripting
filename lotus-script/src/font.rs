use lotus_script_sys::FfiObject;
use lotus_shared::content::ContentId;
pub use lotus_shared::font::*;

/// A bitmap font that can be used to render text.
pub struct BitmapFont(ContentId);

impl BitmapFont {
    /// Create a new bitmap font from a content id.
    pub fn new(content_id: ContentId) -> Self {
        Self(content_id)
    }

    /// Get the properties of this font.
    /// Returns `None` if the font is not currently loaded. It will be loaded in the background.
    /// Just call this function again later until it returns `Some`.
    pub fn properties(&self) -> Option<BitmapFontProperties> {
        let font = FfiObject::new(&self.0);
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
    pub fn text_len(&self, text: &str, letter_spacing: i32) -> Option<u32> {
        let font = FfiObject::new(&self.0);
        let text = FfiObject::new(&text);

        let len = unsafe {
            lotus_script_sys::font::text_len(font.packed(), text.packed(), letter_spacing)
        };

        if len < 0 {
            None
        } else {
            Some(len as u32)
        }
    }
}
