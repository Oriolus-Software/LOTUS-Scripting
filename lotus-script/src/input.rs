//! Maus-Eingabe und Tastatur-Aktionstypen.
//!
//! Mouse input and keyboard action types.

use lotus_script_sys::FfiObject;
use lotus_shared::math::Vec2;

pub use lotus_shared::input::*;

/// Gibt die Mausbewegung seit dem letzten Frame zurück.
///
/// Returns the mouse movement since the last frame.
pub fn mouse_delta() -> Vec2 {
    let delta = unsafe { lotus_script_sys::input::mouse_delta() };
    FfiObject::from_packed(delta).deserialize()
}

/// Gibt die aktuelle Mausposition zurück.
///
/// Returns the current mouse position.
pub fn mouse_position() -> Vec2 {
    let delta = unsafe { lotus_script_sys::input::mouse_position() };
    FfiObject::from_packed(delta).deserialize()
}

/// Modus der Mauslenkung für Straßenfahrzeuge.
///
/// Mouse steering mode for road vehicles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MouseSteeringMode {
    /// Mauslenkung inaktiv.
    ///
    /// Mouse steering inactive.
    #[default]
    Inactive,
    /// Mauslenkung aktiv.
    ///
    /// Mouse steering active.
    Active,
    /// Mauslenkung aktiv mit verstärkter Empfindlichkeit.
    ///
    /// Mouse steering active with boosted sensitivity.
    ActiveBoost,
}

impl From<u32> for MouseSteeringMode {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Active,
            2 => Self::ActiveBoost,
            _ => Self::Inactive,
        }
    }
}

/// Gibt den aktuellen Mauslenkungsmodus zurück.
///
/// Returns the current mouse steering mode.
pub fn mouse_steering_mode() -> MouseSteeringMode {
    let active = unsafe { lotus_script_sys::input::mouse_steering_mode() };
    active.into()
}
