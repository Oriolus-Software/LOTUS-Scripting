pub use lotus_shared::graphics::*;

pub mod textures {
    use lotus_script_sys::FfiObject;
    use lotus_shared::{graphics::Color, math::UVec2};

    pub use lotus_shared::graphics::textures::*;

    /// A texture that can be manipulated and displayed on script texture slots.
    #[derive(Debug)]
    pub struct Texture(TextureHandle);

    impl Texture {
        /// Create a new texture.
        #[must_use]
        pub fn create(options: TextureCreationOptions) -> Self {
            let options = FfiObject::new(&options);

            unsafe {
                Self(TextureHandle::new(lotus_script_sys::textures::create(
                    options.packed(),
                )))
            }
        }

        /// Add an action to the texture. You may want to call the helper methods
        /// instead of this.
        pub fn add_action(&mut self, action: TextureAction) {
            let action = FfiObject::new(&action);

            unsafe { lotus_script_sys::textures::add_action(self.0.id(), action.packed()) }
        }

        /// Draw a rectangle on the texture.
        pub fn draw_rect(&mut self, start: impl Into<UVec2>, end: impl Into<UVec2>, color: Color) {
            self.add_action(TextureAction::DrawRect {
                start: start.into(),
                end: end.into(),
                color,
            });
        }

        /// Clear the texture with a color.
        pub fn clear(&mut self, color: Color) {
            self.add_action(TextureAction::Clear(color));
        }

        /// Get the color of a pixel on the texture.
        #[inline]
        pub fn get_pixel(&self, x: u32, y: u32) -> Color {
            let packed = unsafe { lotus_script_sys::textures::get_pixel(self.0.id(), x, y) };
            packed.into()
        }

        /// For better performance, use [`Self::set_pixels`] instead with all the pixels you want to set.
        /// This is only here for convenience.
        pub fn set_pixel<P>(&mut self, draw_pixel: P)
        where
            P: Into<DrawPixel> + Copy,
        {
            self.set_pixels(&[draw_pixel]);
        }

        /// Set multiple pixels on the texture.
        pub fn set_pixels<P>(&mut self, pixels: &[P])
        where
            P: Into<DrawPixel> + Copy,
        {
            let pixels = pixels.iter().map(|p| (*p).into()).collect();

            self.add_action(TextureAction::DrawPixels(pixels));
        }

        /// Draws another texture on top of this one.
        pub fn draw_texture(&mut self, other: &Texture, options: DrawTextureOpts) {
            self.add_action(TextureAction::DrawScriptTexture {
                handle: other.handle(),
                options,
            });
        }

        /// Call this once for every game-texture you want to apply this to. You can define the name in the content tool.
        pub fn apply_to(&mut self, name: &str) {
            let name = FfiObject::new(&name);
            unsafe { lotus_script_sys::textures::apply_to(self.0.id(), name.packed()) }
        }

        /// Only call this if you need your actions to be applied immediately.
        /// Cause of streaming assets, this method will return false if the actions are not yet applied.
        /// Just call this method again until it returns true.
        pub fn flush(&mut self) -> bool {
            unsafe { lotus_script_sys::textures::flush_actions(self.0.id()) == 1 }
        }

        /// Get the handle of the texture.
        pub fn handle(&self) -> TextureHandle {
            self.0
        }
    }

    impl Drop for Texture {
        fn drop(&mut self) {
            unsafe { lotus_script_sys::textures::dispose(self.0.id()) }
        }
    }

    pub enum DrawableTexture {
        // TODO: Support content textures.
        // Content(ContentId),
        Script(TextureHandle),
    }

    // impl From<ContentId> for DrawableTexture {
    //     fn from(content: ContentId) -> Self {
    //         Self::Content(content)
    //     }
    // }

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
