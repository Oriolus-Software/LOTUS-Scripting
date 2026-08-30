//! Bitmap-Schriftarten zum Rendern von Text auf Script-Texturen.
//!
//! Bitmap fonts for rendering text on script textures.

use lotus_script_sys::FfiObject;
use lotus_shared::content::ContentId;
pub use lotus_shared::font::*;

/// Bitmap-Schrift, die zum Rendern von Text verwendet werden kann.
///
/// A bitmap font that can be used to render text.
pub struct BitmapFont {
    content_id: ContentId,
    properties: BitmapFontProperties,
}

impl BitmapFont {
    /// Versucht, eine Bitmap-Schrift anhand einer Content-ID zu laden.
    /// Gibt `None` zurück, solange das Asset noch im Hintergrund lädt — erneut aufrufen, bis `Some` zurückkommt.
    ///
    /// Tries to load a bitmap font from a content id.
    /// Returns `None` while the asset is still loading in the background; call again until `Some` is returned.
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

    /// Gibt die Eigenschaften dieser Schrift zurück.
    ///
    /// Returns the properties of this font.
    pub fn properties(&self) -> &BitmapFontProperties {
        &self.properties
    }

    /// Gibt die Textbreite in Pixeln zurück.
    ///
    /// Returns the width of the text in pixels.
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
