pub use lotus_shared::graphics::*;

pub mod textures {
    use lotus_script_sys::FfiObject;
    use lotus_shared::{content::ContentId, graphics::Color, math::UVec2};

    pub use lotus_shared::graphics::textures::*;

    pub struct Texture(TextureHandle);

    impl Texture {
        #[must_use]
        pub fn create(options: TextureCreationOptions) -> Self {
            let options = FfiObject::new(&options);

            unsafe {
                Self(TextureHandle::new(lotus_script_sys::textures::create(
                    options.packed(),
                )))
            }
        }

        pub fn add_action(&mut self, action: TextureAction) {
            let action = FfiObject::new(&action);

            unsafe { lotus_script_sys::textures::add_action(self.0.id(), action.packed()) }
        }

        /// Draws a rectangle.
        pub fn draw_rect(&mut self, start: impl Into<UVec2>, end: impl Into<UVec2>, color: Color) {
            self.add_action(TextureAction::DrawRect {
                start: start.into(),
                end: end.into(),
                color,
            });
        }

        /// Draws a texture or script texture.
        pub fn draw_texture(
            &mut self,
            texture: impl Into<DrawableTexture>,
            options: DrawTextureOpts,
        ) {
            match texture.into() {
                // DrawableTexture::Content(texture) => {
                //     self.add_action(TextureAction::DrawTexture { texture, options });
                // }
                DrawableTexture::Script(handle) => {
                    self.add_action(TextureAction::DrawScriptTexture { handle, options });
                }
            }
        }

        /// Clears the texture with the given color.
        pub fn clear(&mut self, color: Color) {
            self.add_action(TextureAction::Clear(color));
        }

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

        pub fn set_pixels<P>(&mut self, pixels: &[P])
        where
            P: Into<DrawPixel> + Copy,
        {
            let pixels = pixels.iter().map(|p| (*p).into()).collect();

            self.add_action(TextureAction::DrawPixels(pixels));
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

        pub fn handle(&self) -> TextureHandle {
            self.0
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
