pub use lotus_shared::graphics::*;

pub mod textures {
    use lotus_script_sys::FfiObject;
    use lotus_shared::{graphics::Color, math::UVec2};

    pub use lotus_shared::graphics::textures::*;

    pub struct Texture(u64);

    impl Texture {
        #[must_use]
        pub fn create(options: TextureCreationOptions) -> Self {
            let options = FfiObject::new(&options);

            unsafe { Self(lotus_script_sys::textures::create(options.packed())) }
        }

        pub fn add_action(&mut self, action: TextureAction) {
            let action = FfiObject::new(&action);

            unsafe { lotus_script_sys::textures::add_action(self.0, action.packed()) }
        }

        pub fn draw_rect(&mut self, start: impl Into<UVec2>, end: impl Into<UVec2>, color: Color) {
            self.add_action(TextureAction::DrawRect(start.into(), end.into(), color));
        }

        pub fn clear(&mut self, color: Color) {
            self.add_action(TextureAction::Clear(color));
        }

        pub fn get_pixel(&self, x: u32, y: u32) -> Color {
            let packed = unsafe { lotus_script_sys::textures::get_pixel(self.0, x, y) };
            packed.into()
        }

        /// For better performance, use [set_pixels] instead with all the pixels you want to set.
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

        pub fn apply_to(&mut self, name: &str) {
            let name = FfiObject::new(&name);
            unsafe { lotus_script_sys::textures::apply_to(self.0, name.packed()) }
        }

        pub fn flush(&mut self) {
            unsafe { lotus_script_sys::textures::flush_actions(self.0) }
        }
    }
}
