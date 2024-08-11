use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::{ffi, FfiObject};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    pub const CYAN: Self = Self::rgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::rgb(1.0, 0.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GizmoKind {
    WireCube { center: Vec3, half_extents: Vec3 },
    WireSphere { center: Vec3, radius: f32 },
    Arrow { start: Vec3, end: Vec3 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gizmo {
    pub kind: GizmoKind,
    pub color: Color,
}

impl Gizmo {
    pub fn draw(&self) {
        let obj = FfiObject::new(self);

        unsafe {
            ffi::gizmo::draw(obj.packed());
        }
    }
}
