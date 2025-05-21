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
/// Gizmos are drawn relative to the center of the object the script is attached to.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gizmo {
    pub kind: GizmoKind,
    pub color: Color,
}

impl Gizmo {
    /// Create a new gizmo.
    pub fn new(kind: GizmoKind, color: Color) -> Self {
        Self { kind, color }
    }

    /// Create a new wire cube gizmo.
    pub fn wire_cube(center: impl Into<Vec3>, half_extents: impl Into<Vec3>, color: Color) -> Self {
        Self::new(
            GizmoKind::WireCube {
                center: center.into(),
                half_extents: half_extents.into(),
            },
            color,
        )
    }

    /// Create a new wire sphere gizmo.
    pub fn wire_sphere(center: impl Into<Vec3>, radius: impl Into<f32>, color: Color) -> Self {
        Self::new(
            GizmoKind::WireSphere {
                center: center.into(),
                radius: radius.into(),
            },
            color,
        )
    }

    /// Create a new arrow gizmo.
    pub fn arrow(start: impl Into<Vec3>, end: impl Into<Vec3>, color: Color) -> Self {
        Self::new(
            GizmoKind::Arrow {
                start: start.into(),
                end: end.into(),
            },
            color,
        )
    }

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
