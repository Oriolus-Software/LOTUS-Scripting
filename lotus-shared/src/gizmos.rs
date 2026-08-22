//! Debug-Gizmos, die relativ zum scriptbehafteten Objekt gezeichnet werden.
//!
//! Debug gizmos drawn relative to the scripted object.

use serde::{Deserialize, Serialize};

use crate::{graphics::Color, math::Vec3};

/// Form eines Debug-Gizmos.
///
/// Shape of a debug gizmo.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GizmoKind {
    /// Drahtgitter-Quader, achsenausgerichtet.
    ///
    /// Wireframe axis-aligned box.
    WireCube { center: Vec3, half_extents: Vec3 },
    /// Drahtgitter-Kugel.
    ///
    /// Wireframe sphere.
    WireSphere { center: Vec3, radius: f32 },
    /// Pfeil von Start- zu Endpunkt.
    ///
    /// Arrow from start to end.
    Arrow { start: Vec3, end: Vec3 },
}

/// Debug-Gizmo zur Visualisierung im Spiel.
/// Gizmos sind visuelle Hilfen für Debugging und Entwicklung
/// und werden relativ zum Mittelpunkt des scriptbehafteten Objekts gezeichnet.
///
/// A gizmo to draw in the game.
/// A gizmo is a visual indicator of something in the game.
/// It is used to help with debugging and development.
/// Gizmos are drawn relative to the center of the object the script is attached to.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gizmo {
    /// Form und Geometrie des Gizmos.
    ///
    /// Shape and geometry of the gizmo.
    pub kind: GizmoKind,
    /// Farbe beim Zeichnen des Gizmos.
    ///
    /// Color used when drawing the gizmo.
    pub color: Color,
}

impl Gizmo {
    /// Erstellt ein neues Gizmo.
    ///
    /// Create a new gizmo.
    pub fn new(kind: GizmoKind, color: Color) -> Self {
        Self { kind, color }
    }

    /// Erstellt ein neues Drahtgitter-Quader-Gizmo.
    ///
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

    /// Erstellt ein neues Drahtgitter-Kugel-Gizmo.
    ///
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

    /// Erstellt ein neues Pfeil-Gizmo.
    ///
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

    /// Zeichnet das Gizmo.
    ///
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
