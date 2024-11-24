use serde::{Deserialize, Serialize};

pub use glam::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rectangle {
    start: UVec2,
    end: UVec2,
}

impl Rectangle {
    pub fn new(start: UVec2, end: UVec2) -> Self {
        assert!(start.x <= end.x);
        assert!(start.y <= end.y);

        Self { start, end }
    }

    pub fn from_size(start: UVec2, size: UVec2) -> Self {
        Self::new(start, start + size)
    }

    #[inline(always)]
    pub fn start(&self) -> UVec2 {
        self.start
    }

    #[inline(always)]
    pub fn end(&self) -> UVec2 {
        self.end
    }

    #[inline(always)]
    pub fn width(&self) -> u32 {
        self.end.x - self.start.x
    }

    #[inline(always)]
    pub fn height(&self) -> u32 {
        self.end.y - self.start.y
    }

    #[inline(always)]
    pub fn size(&self) -> UVec2 {
        UVec2 {
            x: self.width(),
            y: self.height(),
        }
    }

    #[inline]
    pub fn contains(&self, point: UVec2) -> bool {
        self.start.x <= point.x
            && point.x <= self.end.x
            && self.start.y <= point.y
            && point.y <= self.end.y
    }

    #[inline]
    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.start.x <= other.end.x
            && self.end.x >= other.start.x
            && self.start.y <= other.end.y
            && self.end.y >= other.start.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_creation() {
        let start = UVec2 { x: 0, y: 0 };
        let end = UVec2 { x: 5, y: 5 };
        let rect = Rectangle::new(start, end);
        assert_eq!(rect.start(), start);
        assert_eq!(rect.end(), end);
    }

    #[test]
    fn test_rectangle_width_and_height() {
        let start = UVec2 { x: 1, y: 1 };
        let end = UVec2 { x: 4, y: 6 };
        let rect = Rectangle::new(start, end);
        assert_eq!(rect.width(), 3);
        assert_eq!(rect.height(), 5);
    }

    #[test]
    fn test_rectangle_contains() {
        let start = UVec2 { x: 0, y: 0 };
        let end = UVec2 { x: 10, y: 10 };
        let rect = Rectangle::new(start, end);
        assert!(rect.contains(UVec2 { x: 5, y: 5 }));
        assert!(!rect.contains(UVec2 { x: 11, y: 5 }));
    }

    #[test]
    fn test_rectangle_intersects() {
        let rect1 = Rectangle::new(UVec2 { x: 0, y: 0 }, UVec2 { x: 5, y: 5 });
        let rect2 = Rectangle::new(UVec2 { x: 3, y: 3 }, UVec2 { x: 7, y: 7 });
        let rect3 = Rectangle::new(UVec2 { x: 6, y: 6 }, UVec2 { x: 8, y: 8 });

        assert!(rect1.intersects(&rect2));
        assert!(!rect1.intersects(&rect3));
    }
}
