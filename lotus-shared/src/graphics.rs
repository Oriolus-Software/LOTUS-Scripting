use serde::{Deserialize, Serialize};

/// A color in the RGBA format.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let r = ((value >> 24) & 0xFF) as u8;
        let g = ((value >> 16) & 0xFF) as u8;
        let b = ((value >> 8) & 0xFF) as u8;
        let a = (value & 0xFF) as u8;

        Color::rgba(r, g, b, a)
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        let r = value.r as u32;
        let g = value.g as u32;
        let b = value.b as u32;
        let a = value.a as u32;

        (r << 24) | (g << 16) | (b << 8) | a
    }
}

#[cfg(feature = "bevy")]
mod _bevy {
    use super::*;

    impl From<bevy::color::Color> for Color {
        fn from(value: bevy::color::Color) -> Self {
            let value = value.to_srgba();

            Self::rgba(
                (value.red * 255.0) as u8,
                (value.green * 255.0) as u8,
                (value.blue * 255.0) as u8,
                (value.alpha * 255.0) as u8,
            )
        }
    }

    impl From<Color> for bevy::color::Color {
        fn from(value: Color) -> Self {
            bevy::color::Color::srgba(
                value.r as f32 / 255.0,
                value.g as f32 / 255.0,
                value.b as f32 / 255.0,
                value.a as f32 / 255.0,
            )
        }
    }
}

#[cfg(feature = "image")]
mod _image {
    use super::*;

    impl From<image::Rgba<u8>> for Color {
        fn from(value: image::Rgba<u8>) -> Self {
            Self::rgba(value[0], value[1], value[2], value[3])
        }
    }

    impl From<Color> for image::Rgba<u8> {
        fn from(value: Color) -> Self {
            [value.r, value.g, value.b, value.a].into()
        }
    }
}

pub mod textures {
    use std::borrow::Cow;

    use serde::{Deserialize, Serialize};

    use crate::{
        content::ContentId,
        math::{Rectangle, UVec2},
    };

    use super::Color;

    /// Options for creating a texture.
    #[derive(Clone, Serialize, Deserialize)]
    pub struct TextureCreationOptions<'a> {
        /// The width of the texture.
        pub width: u32,
        /// The height of the texture.
        pub height: u32,
        /// The data of the texture. This is currently a placeholder for future use.
        pub data: Option<Cow<'a, [u8]>>,
    }

    /// A handle to a texture.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct TextureHandle(u32);

    #[cfg(feature = "internal")]
    impl TextureHandle {
        /// Create a new texture handle.
        pub fn new(value: u32) -> Self {
            Self(value)
        }

        /// Get the ID of the texture handle.
        pub fn id(&self) -> u32 {
            self.0
        }
    }

    /// An action to perform on a texture.
    #[derive(Clone, Serialize, Deserialize)]
    pub enum TextureAction {
        /// Clear the texture with a color.
        Clear(Color),
        /// Draw pixels on the texture.
        DrawPixels(Box<[DrawPixel]>),
        /// Draw a rectangle on the texture.
        DrawRect {
            start: UVec2,
            end: UVec2,
            color: Color,
        },
        /// Draw text on the texture.
        DrawText {
            font: ContentId,
            text: String,
            top_left: UVec2,
            letter_spacing: u32,
            full_color: Option<Color>,
        },
        // DrawTexture {
        //     texture: ContentId,
        //     options: DrawTextureOpts,
        // },
        /// Draw a script texture on the texture.
        DrawScriptTexture {
            handle: TextureHandle,
            options: DrawTextureOpts,
        },
    }

    /// Options for drawing a texture.
    #[derive(Default, Clone, Copy, Serialize, Deserialize)]
    pub struct DrawTextureOpts {
        /// The source rectangle of the texture to draw.
        pub source_rect: Option<Rectangle>,
        /// The target rectangle of the texture to draw to.
        pub target_rect: Option<Rectangle>,
    }

    /// A pixel to draw on a texture.
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub struct DrawPixel {
        /// The position of the pixel.
        pub pos: UVec2,
        /// The color of the pixel.
        pub color: Color,
    }

    impl From<(UVec2, Color)> for DrawPixel {
        fn from((position, color): (UVec2, Color)) -> Self {
            Self {
                pos: position,
                color,
            }
        }
    }

    impl From<(u32, u32, Color)> for DrawPixel {
        fn from((x, y, color): (u32, u32, Color)) -> Self {
            Self {
                pos: UVec2 { x, y },
                color,
            }
        }
    }
}
