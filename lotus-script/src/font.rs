use lotus_script_sys::FfiObject;
use lotus_shared::content::ContentId;
pub use lotus_shared::font::*;

/// A bitmap font that can be used to render text.
pub struct BitmapFont {
    content_id: ContentId,
    properties: BitmapFontProperties,
}

impl BitmapFont {
    /// Try to load a bitmap font from a content id.
    /// Returns `None` if the font is not currently loaded. It will be loaded in the background.
    /// Just call this function again later until it returns `Some`.
    pub fn try_load(content_id: ContentId) -> Option<Self> {
        let font = FfiObject::new(&content_id);
        let properties = unsafe { lotus_script_sys::font::bitmap_font_properties(font.packed()) };

        if properties == 0 {
            None
        } else {
            let properties = FfiObject::from_packed(properties).deserialize();
            Some(Self {
                content_id,
                properties,
            })
        }
    }

    /// Get the properties of this font.
    pub fn properties(&self) -> &BitmapFontProperties {
        &self.properties
    }

    /// Get the width of the text in pixels.
    pub fn text_len(&self, text: &str, letter_spacing: i32) -> u32 {
        let font = FfiObject::new(&self.content_id);
        let text = FfiObject::new(&text);

        let len = unsafe {
            lotus_script_sys::font::text_len(font.packed(), text.packed(), letter_spacing)
        };

        assert!(len >= 0);

        len as u32
    }
}
