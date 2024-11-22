use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitmapFontProperties {
    pub horizontal_distance: i32,
    pub vertical_size: i32,
    pub letters: HashMap<char, FontLetter>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FontLetter {
    pub character: char,
    pub start: u32,
    pub width: u32,
}
