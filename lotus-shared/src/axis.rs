//! Logische Controller-Achsen und Force Feedback, getrennt von Tastenaktionen.
//!
//! Logical controller axes and force feedback, separate from button actions.

use serde::{Deserialize, Serialize};

/// Achsenwert ohne Belegung und Ruhelage eines zentrierten Gebers.
///
/// Axis value without a binding, and the rest position of a centered control.
pub const AXIS_CENTER: f32 = 0.5;

/// Beschreibt eine Achse, die bei der Engine registriert werden kann.
///
/// Describes an axis that can be registered with the engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterAxis {
    /// Eindeutige Achsen-ID für Scripts und die Belegung.
    ///
    /// Unique axis identifier used by scripts and bindings.
    pub id: String,
}

impl RegisterAxis {
    /// Erstellt einen neuen Registrierungseintrag für eine Achse.
    ///
    /// Creates a new axis registration entry.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

/// Rückstellkraft einer Achse.
///
/// Restoring force of an axis.
///
/// `coefficient` und `saturation` sind unbeschränkte endliche Stärken.
/// Der Nennbereich ist `0..=1`. Die Engine multipliziert mit der Geräte-Intensität
/// und klemmt erst danach auf `0..=1`.
/// `offset` verschiebt die Federmitte und wird danach auf `-1..=1` geklemmt, ohne Skalierung.
/// `0` lässt die Federmitte unverändert.
///
/// `coefficient` and `saturation` are unrestricted finite strengths.
/// The nominal range is `0..=1`. The engine multiplies by the device intensity
/// and only then clamps to `0..=1`.
/// `offset` shifts the spring center and is then clamped to `-1..=1`, without scaling.
/// `0` leaves the spring center unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpringForce {
    /// Steifigkeit der Feder.
    ///
    /// Spring stiffness.
    pub coefficient: f32,
    /// Obere Grenze der Kraft.
    ///
    /// Upper bound of the force.
    pub saturation: f32,
    /// Verschiebung der Federmitte.
    ///
    /// Shift of the spring center.
    pub offset: f32,
}

/// Schwingung entlang einer Achse.
///
/// Oscillation along an axis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AxisVibration {
    /// Stärke als unbeschränkter endlicher Wert. `0` schaltet die Schwingung aus.
    /// Die Engine multipliziert mit der Geräte-Intensität und klemmt erst danach auf `0..=1`.
    ///
    /// Strength as an unrestricted finite value. `0` turns the oscillation off.
    /// The engine multiplies by the device intensity and only then clamps to `0..=1`.
    pub magnitude: f32,
    /// Frequenz in Hertz.
    ///
    /// Frequency in hertz.
    pub frequency_hz: f32,
}

/// Zuletzt gewünschtes Force Feedback einer logischen Achse.
///
/// Latest requested force feedback for one logical axis.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct AxisForceFeedback {
    /// Feder. `None`, solange das Script noch keine gesetzt hat.
    ///
    /// Spring. `None` until the script sets one.
    pub spring: Option<SpringForce>,
    /// Reibung, `0` heißt aus.
    ///
    /// Friction; `0` means off.
    pub friction: f32,
    /// Schwingung. `None` heißt aus.
    ///
    /// Oscillation. `None` means off.
    pub vibration: Option<AxisVibration>,
}

/// Begrenzt einen endlichen Wert auf `0..=1`. Nicht endliche Werte werden 0.
///
/// Clamps a finite value to `0..=1`. Non-finite values become 0.
pub fn clamp_unit(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// Begrenzt einen endlichen Wert auf `-1..=1`. Nicht endliche Werte werden 0.
///
/// Clamps a finite value to `-1..=1`. Non-finite values become 0.
pub fn clamp_signed_unit(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(-1.0, 1.0)
    } else {
        0.0
    }
}

/// Begrenzt eine Frequenz auf `>= 0`. Nicht endliche Werte werden 0.
///
/// Clamps a frequency to `>= 0`. Non-finite values become 0.
pub fn clamp_frequency_hz(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
