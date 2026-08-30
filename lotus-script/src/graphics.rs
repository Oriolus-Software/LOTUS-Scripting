//! Zeichen- und Textur-API für Script-Displays.
//!
//! Drawing and texture API for script displays.

#[cfg(feature = "internal")]
use lotus_script_sys::FfiObject;

pub use lotus_shared::graphics::*;

pub mod textures {
    //! Erstellung und Bearbeitung von Script-Texturen.
    //!
    //! Creation and manipulation of script textures.

    use lotus_script_sys::FfiObject;
    use lotus_shared::{
        content::ContentId,
        graphics::Color,
        math::{IVec2, Rectangle, UVec2},
    };

    pub use lotus_shared::graphics::textures::*;

    /// Textur, die bearbeitet und auf Script-Textur-Slots angezeigt werden kann.
    ///
    /// A texture that can be manipulated and displayed on script texture slots.
    #[derive(Debug)]
    pub struct Texture(TextureHandle);

    impl Texture {
        /// Erstellt eine neue Script-Textur.
        ///
        /// Creates a new script texture.
        #[must_use]
        pub fn create<'a>(options: impl Into<TextureCreationOptions<'a>>) -> Self {
            let options = options.into();
            let options = FfiObject::new(&options);

            unsafe {
                Self(TextureHandle::new(lotus_script_sys::textures::create(
                    options.packed(),
                )))
            }
        }

        /// Fügt eine Zeichenaktion zur Textur hinzu.
        /// Bevorzugt die Hilfsmethoden statt direktem Aufruf.
        ///
        /// Adds a drawing action to the texture.
        /// Prefer the helper methods over calling this directly.
        pub fn add_action(&mut self, action: TextureAction) {
            let action = FfiObject::new(&action);

            unsafe { lotus_script_sys::textures::add_action(self.0.id(), action.packed()) }
        }

        /// Zeichnet ein Rechteck auf die Textur.
        ///
        /// Draws a rectangle on the texture.
        pub fn draw_rect(&mut self, start: impl Into<UVec2>, end: impl Into<UVec2>, color: Color) {
            self.add_action(TextureAction::DrawRect {
                start: start.into(),
                end: end.into(),
                color,
            });
        }

        /// Füllt die Textur mit einer Farbe.
        ///
        /// Clears the texture with a color.
        pub fn clear(&mut self, color: Color) {
            self.add_action(TextureAction::Clear(color));
        }

        /// Liest die Farbe eines Pixels auf der Textur.
        ///
        /// Reads the color of a pixel on the texture.
        #[inline]
        pub fn read_pixel(&self, x: u32, y: u32) -> Color {
            let packed = unsafe { lotus_script_sys::textures::get_pixel(self.0.id(), x, y) };
            packed.into()
        }

        /// Zeichnet mehrere Pixel auf die Textur.
        ///
        /// Draws multiple pixels on the texture.
        pub fn draw_pixels<P>(&mut self, pixels: &[P])
        where
            P: Into<DrawPixel> + Copy,
        {
            let pixels = pixels.iter().map(|p| (*p).into()).collect();

            self.add_action(TextureAction::DrawPixels(pixels));
        }

        /// Zeichnet eine andere Textur über diese.
        ///
        /// Draws another texture on top of this one.
        pub fn draw_texture(&mut self, other: &Texture, options: DrawTextureOpts) {
            self.add_action(TextureAction::DrawScriptTexture {
                handle: other.handle(),
                options,
            });
        }

        /// Wendet die Textur auf einen benannten Spiel-Textur-Slot an (Name im Content-Tool).
        /// Einmal pro Ziel-Slot aufrufen.
        ///
        /// Applies the texture to a named in-game texture slot (name defined in the content tool).
        /// Call once per target slot.
        pub fn apply_to(&mut self, name: &str) {
            let name = FfiObject::new(&name);
            unsafe { lotus_script_sys::textures::apply_to(self.0.id(), name.packed()) }
        }

        /// Wendet ausstehende Aktionen sofort an.
        /// Kann `false` liefern, solange Assets noch streamen — erneut aufrufen, bis `true`.
        ///
        /// Applies pending actions immediately.
        /// May return `false` while assets are still streaming; call again until `true`.
        pub fn flush(&mut self) -> bool {
            unsafe { lotus_script_sys::textures::flush_actions(self.0.id()) == 1 }
        }

        /// Zeichnet eine andere Script-Textur über diese.
        ///
        /// Draws another script texture on top of this one.
        pub fn draw_script_texture(&mut self, other: &Texture, options: DrawTextureOpts) {
            self.add_action(TextureAction::DrawScriptTexture {
                handle: other.handle(),
                options,
            });
        }

        /// Zeichnet Text auf die Textur.
        ///
        /// Draws text on the texture.
        #[expect(clippy::too_many_arguments)]
        pub fn draw_text(
            &mut self,
            font: ContentId,
            text: impl Into<String>,
            top_left: impl Into<IVec2>,
            letter_spacing: u32,
            full_color: impl Into<Option<Color>>,
            alpha_mode: AlphaMode,
            target_rect: impl Into<Option<Rectangle>>,
        ) {
            self.add_action(TextureAction::DrawText {
                font,
                text: text.into(),
                top_left: top_left.into(),
                letter_spacing,
                full_color: full_color.into(),
                alpha_mode,
                target_rect: target_rect.into(),
            });
        }

        /// Gibt den Handle der Textur zurück.
        ///
        /// Returns the handle of the texture.
        pub fn handle(&self) -> TextureHandle {
            self.0
        }

        /// Verhindert das Freigeben beim Drop — Textur bleibt ohne Referenz bestehen.
        /// Nur verwenden, wenn die Textur absichtlich ohne Handle weiterleben soll.
        ///
        /// Prevents disposal on drop so the texture outlives this handle.
        /// Use only when the texture should stay alive without a reference.
        pub fn forget(mut self) {
            self.0 = TextureHandle::new(u32::MAX);
        }

        /// Stellt die Textur unter dem angegebenen Namen für die Plugin-API bereit.
        ///
        /// Exposes the texture to the plugin API under the given name.
        pub fn expose(&self, name: &str) {
            let name = FfiObject::new(&name);
            unsafe { lotus_script_sys::textures::expose(self.0.id(), name.packed()) }
        }
    }

    impl Drop for Texture {
        fn drop(&mut self) {
            if self.0.id() != u32::MAX {
                unsafe { lotus_script_sys::textures::dispose(self.0.id()) }
            }
        }
    }

    /// Zeichenbare Textur aus Script-Handle oder (künftig) Content-ID.
    ///
    /// Drawable texture from a script handle or (future) content id.
    pub enum DrawableTexture {
        // TODO: Support content textures.
        // Content(ContentId),
        /// Script-Textur-Handle.
        ///
        /// Script texture handle.
        Script(TextureHandle),
    }

    impl From<&Texture> for DrawableTexture {
        fn from(texture: &Texture) -> Self {
            Self::Script(texture.handle())
        }
    }

    impl From<TextureHandle> for DrawableTexture {
        fn from(handle: TextureHandle) -> Self {
            Self::Script(handle)
        }
    }
}

/// Liest Eigenschaften aller zeichenbaren Textur-Slots aus dem Content-Tool.
///
/// Fetches properties of all drawable texture slots from the content tool.
#[cfg(feature = "internal")]
pub fn fetch_drawable_texture_properties() -> Vec<DrawableTextureProperties> {
    let properties = unsafe { lotus_script_sys::textures::fetch_drawable_texture_properties() };
    FfiObject::from_packed(properties).deserialize()
}
