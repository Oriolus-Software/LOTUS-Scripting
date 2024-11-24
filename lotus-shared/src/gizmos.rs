use serde::{Deserialize, Serialize};

use crate::{graphics::Color, math::Vec3};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GizmoKind {
    WireCube { center: Vec3, half_extents: Vec3 },
    WireSphere { center: Vec3, radius: f32 },
    Arrow { start: Vec3, end: Vec3 },
}

/// A gizmo to draw in the game.
/// A gizmo is a visual indicator of something in the game.
/// It is used to help with debugging and development.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gizmo {
    pub kind: GizmoKind,
    pub color: Color,
}

impl Gizmo {
    /// Draw the gizmo.
    #[cfg(feature = "ffi")]
    pub fn draw(&self) {
        use lotus_script_sys::FfiObject;

        let obj = FfiObject::new(self);

        unsafe {
            lotus_script_sys::gizmo::draw(obj.packed());
        }
    }
}
