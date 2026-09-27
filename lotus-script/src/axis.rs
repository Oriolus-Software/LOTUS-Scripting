//! Logische Controller-Achsen und Force Feedback.
//!
//! Logical controller axes and force feedback.

use lotus_script_sys::FfiObject;
pub use lotus_shared::axis::*;

/// Gibt den aktuellen Achsenwert zurück, im Bereich `0..=1`.
/// Ohne Belegung oder vor dem ersten Messwert ist das [`AXIS_CENTER`] (0,5).
///
/// Returns the current axis value, in the range `0..=1`.
/// Without a binding, or before the first sample, this is [`AXIS_CENTER`] (0.5).
pub fn value(id: &str) -> f32 {
    let id = FfiObject::new(&id);
    unsafe { lotus_script_sys::axis::value(id.packed()) }
}

/// Setzt die Rückstellkraft der Achse.
/// Koeffizient 0 schaltet die Feder aus. Werte außerhalb `0..=1` klemmt die Engine.
///
/// Sets the restoring force of the axis.
/// A coefficient of 0 turns the spring off. The engine clamps values outside `0..=1`.
pub fn center_force(id: &str, coefficient: f32, saturation: f32, offset: f32) {
    let id = FfiObject::new(&id);
    unsafe {
        lotus_script_sys::axis::center_force(id.packed(), coefficient, saturation, offset);
    }
}

/// Setzt die Reibung der Achse. 0 schaltet sie aus.
///
/// Sets the friction of the axis. 0 turns it off.
pub fn friction(id: &str, coefficient: f32) {
    let id = FfiObject::new(&id);
    unsafe { lotus_script_sys::axis::friction(id.packed(), coefficient) }
}

/// Setzt die Schwingung der Achse. Magnitude 0 schaltet sie aus.
///
/// Sets the oscillation of the axis. A magnitude of 0 turns it off.
pub fn vibration(id: &str, magnitude: f32, frequency_hz: f32) {
    let id = FfiObject::new(&id);
    unsafe { lotus_script_sys::axis::vibration(id.packed(), magnitude, frequency_hz) }
}

/// Registriert Achsen bei der Engine.
/// Aufruf aus [`crate::Script::init`] oder am Anfang von [`crate::Script::actions`].
/// [`crate::script`] ruft das nicht auf, bestehende Addon-Scripts bleiben unverändert.
///
/// Registers axes with the engine.
/// Call this from [`crate::Script::init`] or at the start of [`crate::Script::actions`].
/// [`crate::script`] does not call it, so existing addon scripts stay unchanged.
pub fn register_many(axes: &[RegisterAxis]) {
    for axis in axes {
        let axis = FfiObject::new(&axis);
        unsafe {
            lotus_script_sys::axis::register(axis.packed());
        }
    }
}
