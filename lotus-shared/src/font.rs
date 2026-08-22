//! Bitmapfont-Metadaten zum Zeichnen von Text auf Script-Texturen.
//!
//! Bitmap font metadata used when drawing text on script textures.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Eigenschaften einer Bitmap-Schrift.
///
/// Properties of a bitmap font.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitmapFontProperties {
    /// Horizontaler Abstand zwischen den Buchstaben.
    ///
    /// The horizontal distance between letters.
    pub horizontal_distance: i32,
    /// Vertikale Größe der Schrift.
    ///
    /// The vertical size of the font.
    pub vertical_size: i32,
    /// Enthaltene Buchstaben der Schrift.
    ///
    /// The letters in the font.
    pub letters: HashMap<char, FontLetter>,
}

/// Ein Zeichen in einer Bitmap-Schrift.
///
/// A letter in a bitmap font.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FontLetter {
    /// Das durch den Buchstaben dargestellte Zeichen.
    ///
    /// The character represented by the letter.
    pub character: char,
    /// Startposition des Buchstaben in der Textur.
    ///
    /// The start of the letter in the texture.
    pub start: u32,
    /// Breite des Buchstaben in der Textur.
    ///
    /// The width of the letter in the texture.
    pub width: u32,
}
