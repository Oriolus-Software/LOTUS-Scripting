//! Farben und Typen zum Zeichnen auf Script-Texturen.
//!
//! Colors and script texture drawing types.

use serde::{Deserialize, Serialize};

#[cfg(feature = "internal")]
use crate::content::ContentId;

/// Eine Farbe im RGBA-Format.
///
/// A color in the RGBA format.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    /// Rotkanal, 0–255.
    ///
    /// Red channel, 0–255.
    pub r: u8,
    /// Grünkanal, 0–255.
    ///
    /// Green channel, 0–255.
    pub g: u8,
    /// Blaukanal, 0–255.
    ///
    /// Blue channel, 0–255.
    pub b: u8,
    /// Alphakanal, 0–255.
    ///
    /// Alpha channel, 0–255.
    pub a: u8,
}

impl Color {
    /// Deckendes Weiß.
    ///
    /// Opaque white.
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    /// Deckendes Schwarz.
    ///
    /// Opaque black.
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    /// Deckendes Rot.
    ///
    /// Opaque red.
    pub const RED: Self = Self::rgb(255, 0, 0);
    /// Deckendes Grün.
    ///
    /// Opaque green.
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    /// Deckendes Blau.
    ///
    /// Opaque blue.
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    /// Deckendes Gelb.
    ///
    /// Opaque yellow.
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    /// Deckendes Cyan.
    ///
    /// Opaque cyan.
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    /// Deckendes Magenta.
    ///
    /// Opaque magenta.
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);

    /// Erstellt eine deckende RGB-Farbe.
    ///
    /// Creates an opaque RGB color.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Erstellt eine RGBA-Farbe.
    ///
    /// Creates an RGBA color.
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

/// Erstellung und Zeichenbefehle für Script-Texturen.
///
/// Script texture creation and drawing commands.
pub mod textures {
    use std::borrow::Cow;

    use glam::IVec2;
    use serde::{Deserialize, Serialize};

    use crate::{
        content::ContentId,
        math::{Rectangle, UVec2},
    };

    use super::Color;

    /// Optionen zum Erstellen einer Textur.
    ///
    /// Options for creating a texture.
    #[derive(Clone, Serialize, Deserialize)]
    pub struct TextureCreationOptions<'a> {
        /// Breite der Textur.
        ///
        /// The width of the texture.
        pub width: u32,
        /// Höhe der Textur.
        ///
        /// The height of the texture.
        pub height: u32,
        /// Texturdaten; derzeit Platzhalter für künftige Verwendung.
        ///
        /// The data of the texture. This is currently a placeholder for future use.
        pub data: Option<Cow<'a, [u8]>>,
        /// Ob für die Textur Mipmaps erzeugt werden sollen.
        ///
        /// Whether to generate mipmaps for the texture.
        pub mipmaps: bool,
    }

    impl From<(u32, u32)> for TextureCreationOptions<'_> {
        fn from((width, height): (u32, u32)) -> Self {
            Self {
                width,
                height,
                data: None,
                mipmaps: false,
            }
        }
    }

    /// Handle auf eine Textur.
    ///
    /// A handle to a texture.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct TextureHandle(u32);

    #[cfg(feature = "internal")]
    impl TextureHandle {
        /// Erstellt einen neuen Textur-Handle.
        ///
        /// Create a new texture handle.
        pub fn new(value: u32) -> Self {
            Self(value)
        }

        /// Gibt die ID des Textur-Handles zurück.
        ///
        /// Get the ID of the texture handle.
        pub fn id(&self) -> u32 {
            self.0
        }
    }

    /// Aktion, die auf einer Textur ausgeführt wird.
    ///
    /// An action to perform on a texture.
    #[derive(Clone, Serialize, Deserialize)]
    pub enum TextureAction {
        /// Füllt die Textur mit einer Farbe.
        ///
        /// Clear the texture with a color.
        Clear(Color),
        /// Zeichnet Pixel auf die Textur.
        ///
        /// Draw pixels on the texture.
        DrawPixels(Box<[DrawPixel]>),
        /// Zeichnet ein Rechteck auf die Textur.
        ///
        /// Draw a rectangle on the texture.
        DrawRect {
            /// Obere linke Ecke des Rechtecks.
            ///
            /// Top-left corner of the rectangle.
            start: UVec2,
            /// Untere rechte Ecke des Rechtecks.
            ///
            /// Bottom-right corner of the rectangle.
            end: UVec2,
            /// Füllfarbe des Rechtecks.
            ///
            /// Fill color of the rectangle.
            color: Color,
        },
        /// Zeichnet Text auf die Textur.
        ///
        /// Draw text on the texture.
        DrawText {
            /// Schrift zum Rendern des Textes.
            ///
            /// Font used to render the text.
            font: ContentId,
            /// Zu zeichnender Text.
            ///
            /// Text string to draw.
            text: String,
            /// Obere linke Textposition in Pixeln.
            ///
            /// Top-left position of the text in pixels.
            top_left: IVec2,
            /// Zusätzlicher Buchstabenabstand in Pixeln.
            ///
            /// Additional spacing between letters in pixels.
            letter_spacing: u32,
            /// Optionale Überschreibungsfarbe für den gesamten Text.
            ///
            /// Optional override color for the entire text.
            full_color: Option<Color>,
            /// Alpha-Mischmodus für den Text.
            ///
            /// Alpha blending mode for the text.
            alpha_mode: AlphaMode,
            /// Optionales Ziel-/Clip-Rechteck für den Text.
            ///
            /// Optional clipping/target rectangle for the text.
            target_rect: Option<Rectangle>,
        },
        // DrawTexture {
        //     texture: ContentId,
        //     options: DrawTextureOpts,
        // },
        /// Zeichnet eine Script-Textur auf die Textur.
        ///
        /// Draw a script texture on the texture.
        DrawScriptTexture {
            /// Handle der Quell-Script-Textur.
            ///
            /// Handle of the source script texture.
            handle: TextureHandle,
            /// Zeichenoptionen wie Quell- und Zielrechteck.
            ///
            /// Drawing options such as source and target rectangles.
            options: DrawTextureOpts,
        },
    }

    /// Steuert die Alpha-Behandlung beim Zeichnen.
    ///
    /// Controls how alpha (transparency) is handled when drawing.
    #[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
    pub enum AlphaMode {
        /// Textur wird deckend gezeichnet; Alphawerte werden ignoriert.
        /// Gezeichnete Pixel ersetzen vorhandene Pixel vollständig.
        ///
        /// The texture is drawn completely opaque, ignoring alpha values.
        /// Any pixels drawn will completely replace the existing pixels.
        #[default]
        Opaque,
        /// Alphawerte unter dem Schwellwert gelten als vollständig transparent,
        /// Werte ab dem Schwellwert als vollständig deckend.
        /// Der Schwellwert sollte zwischen 0.0 und 1.0 liegen.
        ///
        /// Alpha values below the threshold are considered fully transparent,
        /// while values above or equal to the threshold are considered fully opaque.
        /// The threshold should be between 0.0 and 1.0.
        Mask(f32),
        /// Alphawerte werden zum Mischen mit vorhandenen Pixeln verwendet.
        /// Der Alphakanal bestimmt die Deckkraft jedes gezeichneten Pixels.
        ///
        /// Alpha values are used to blend the new pixels with existing pixels.
        /// The alpha channel determines the opacity of each pixel being drawn.
        Blend,
    }

    /// Optionen zum Zeichnen einer Textur.
    ///
    /// Options for drawing a texture.
    #[derive(Default, Clone, Copy, Serialize, Deserialize)]
    pub struct DrawTextureOpts {
        /// Quellrechteck des zu zeichnenden Texturausschnitts.
        ///
        /// The source rectangle of the texture to draw.
        pub source_rect: Option<Rectangle>,
        /// Zielrechteck auf der Zieltextur.
        ///
        /// The target rectangle of the texture to draw to.
        pub target_rect: Option<Rectangle>,
    }

    /// Ein Pixel, der auf eine Textur gezeichnet wird.
    ///
    /// A pixel to draw on a texture.
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub struct DrawPixel {
        /// Position des Pixels.
        ///
        /// The position of the pixel.
        pub pos: UVec2,
        /// Farbe des Pixels.
        ///
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

/// Eigenschaften eines zeichenbaren Textur-Slots, der an Script-Variablen gebunden ist.
///
/// Properties of a drawable game texture slot bound to script variables.
#[cfg(feature = "internal")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawableTextureProperties {
    /// Texturbreite in Pixeln.
    ///
    /// Texture width in pixels.
    pub width: u32,
    /// Texturhöhe in Pixeln.
    ///
    /// Texture height in pixels.
    pub height: u32,
    /// Variablenname für die Textur-ID bzw. den Handle.
    ///
    /// Variable name holding the texture content id or handle.
    pub texture_variable_id: String,
    /// Schrift zum Zeichnen von Text auf die Textur.
    ///
    /// Font used when drawing text onto the texture.
    pub font: ContentId,
    /// Variablenname für den zu zeichnenden Text.
    ///
    /// Variable name holding the text to draw.
    pub text_variable_id: String,
    /// Ob eine benutzerdefinierte Textfarbe verwendet werden soll.
    ///
    /// Whether a custom text color should be applied.
    pub set_color: bool,
    /// Textfarbe, wenn [`set_color`](Self::set_color) aktiv ist.
    ///
    /// Text color when [`set_color`](Self::set_color) is true.
    pub color: Color,
    /// Horizontale Textausrichtung.
    ///
    /// Horizontal text alignment.
    pub horizontal_alignment: TextHorizontalAlignment,
    /// Vertikale Textausrichtung.
    ///
    /// Vertical text alignment.
    pub vertical_alignment: TextVerticalAlignment,
    /// Auflösung des Ausrichtungsrasters im Content-Tool.
    ///
    /// Alignment grid resolution used by the content tool.
    pub alignment_resolution: u8,
}

/// Horizontale Textausrichtung auf zeichenbaren Texturen.
///
/// Horizontal alignment of text on drawable textures.
#[cfg(feature = "internal")]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum TextHorizontalAlignment {
    /// Horizontal zentriert.
    ///
    /// Centered horizontally.
    #[default]
    Center,
    /// Am linken Rand ausgerichtet.
    ///
    /// Aligned to the left edge.
    Left,
    /// Am rechten Rand ausgerichtet.
    ///
    /// Aligned to the right edge.
    Right,
    /// Ganzzahlig zentriert mit Tendenz nach links.
    ///
    /// Integer-centered with bias to the left.
    IntCenterLeft,
    /// Ganzzahlig zentriert mit Tendenz nach rechts.
    ///
    /// Integer-centered with bias to the right.
    IntCenterRight,
}

/// Vertikale Textausrichtung auf zeichenbaren Texturen.
///
/// Vertical alignment of text on drawable textures.
#[cfg(feature = "internal")]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub enum TextVerticalAlignment {
    /// Vertikal zentriert.
    ///
    /// Centered vertically.
    #[default]
    Center,
    /// Am oberen Rand ausgerichtet.
    ///
    /// Aligned to the top edge.
    Top,
    /// Am unteren Rand ausgerichtet.
    ///
    /// Aligned to the bottom edge.
    Bottom,
}
