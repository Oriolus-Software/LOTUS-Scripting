use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Properties of a bitmap font.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitmapFontProperties {
    /// The horizontal distance between letters.
    pub horizontal_distance: i32,
    /// The vertical size of the font.
    pub vertical_size: i32,
    /// The letters in the font.
    pub letters: HashMap<char, FontLetter>,
}

/// A letter in a bitmap font.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FontLetter {
    /// The character represented by the letter.
    pub character: char,
    /// The start of the letter in the texture.
    pub start: u32,
    /// The width of the letter in the texture.
    pub width: u32,
}
