//! Nachrichten zwischen Fahrzeugscripts und KI-Agenten.
//!
//! Dieselbe Nachricht kann in beide Richtungen laufen: das Nutzerfahrzeug sendet sie
//! an die KI-Agentenverwaltung, ein KI-Agent sendet sie an sein eigenes Fahrzeug.
//! Weitere Nachrichten dieses Protokolls kommen in dieses Modul.
//!
//! Messages between vehicle scripts and AI agents.
//!
//! The same message travels in both directions: the user vehicle sends it to the
//! AI agent management, and an AI agent sends it to its own vehicle.
//! Further messages of this protocol belong in this module.

use serde::{Deserialize, Serialize};

use crate::message::{MessageMeta, MessageType};

/// Stellung des Fahrtrichtungsanzeigers.
///
/// Turn-signal position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Indicator {
    Off,
    Left,
    Right,
    Warn,
}

/// Intensität des Frontlichts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrontLightIntensity {
    Off,
    Park,
    Dim,
    Full,
}

/// Schaltet ein Licht eines Fahrzeugs ein, aus oder auf eine andere Stellung.
///
/// Das Nutzerfahrzeug sendet diese Nachricht an die KI-Agentenverwaltung.
/// Ein KI-Agent sendet sie an sein Fahrzeug, dessen Script das Licht setzt.
///
/// Switches a vehicle light on, off, or to another position.
///
/// The user vehicle sends this message to the AI agent management.
/// An AI agent sends it to its vehicle, whose script applies the light.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Light {
    /// Frontscheinwerfer.
    ///
    /// Headlights.
    Front(FrontLightIntensity),
    /// Schlusslicht.
    ///
    /// Tail lights.
    Tail(bool),
    /// Bremslicht.
    ///
    /// Brake lights.
    Brake(bool),
    /// Fahrtrichtungsanzeiger oder Warnblinkanlage.
    ///
    /// Turn signals or hazard lights.
    Indicator(Indicator),
}

impl MessageType for Light {
    const MESSAGE_META: MessageMeta = MessageMeta::new("agent", "switch_vehicle_light", None);
}
