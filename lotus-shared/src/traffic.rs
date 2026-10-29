//! Ampelphasen, die zwischen Scripts und der Simulator-Engine geteilt werden.
//!
//! Traffic-light phases shared between scripts and the simulator engine.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Script-Variable mit der aktuellen Ampelphase des gebundenen Tracks.
///
/// Script variable for the current traffic-light phase of the bound track.
pub const VAR_TRAFFICLIGHT_PHASE: &str = "trafficlight_phase";
/// Script-Variable mit dem Anforderungszustand des gebundenen Request-Eingangs.
///
/// Script variable for the request flag of the bound request input.
pub const VAR_TRAFFICLIGHT_REQUEST: &str = "trafficlight_request";

/// Aktuelle Phase eines Ampel-Tracks.
///
/// Current phase of a traffic-light track.
///
/// Scripts lesen die Phase mit `get_var::<TrafficLightPhase>(VAR_TRAFFICLIGHT_PHASE)`.
/// Ohne Bindung oder bei ungültigem Wert ist das Ergebnis `None`.
///
/// Scripts read the phase with `get_var::<TrafficLightPhase>(VAR_TRAFFICLIGHT_PHASE)`.
/// Without a binding or for an invalid value the result is `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TrafficLightPhase {
    Red1 = 0,
    Red2 = 1,
    Red3 = 2,
    YellowA1 = 3,
    YellowA2 = 4,
    YellowA3 = 5,
    Green1 = 6,
    Green2 = 7,
    Green3 = 8,
    YellowB1 = 9,
    YellowB2 = 10,
    YellowB3 = 11,
    OffPrimary = 12,
    OffSecondary = 13,
    User1 = 14,
    User2 = 15,
    User3 = 16,
}

impl TrafficLightPhase {
    /// Farben der Pascal-Editor-Zeitleiste.
    ///
    /// Colors matching the Pascal editor timeline.
    pub fn color_rgb(self) -> [u8; 3] {
        match self {
            Self::Red1 | Self::Red2 | Self::Red3 => [255, 0, 0],
            Self::YellowA1 | Self::YellowA2 | Self::YellowA3 => [255, 128, 0],
            Self::Green1 | Self::Green2 | Self::Green3 => [0, 180, 50],
            Self::YellowB1 | Self::YellowB2 | Self::YellowB3 => [255, 255, 0],
            Self::OffPrimary | Self::OffSecondary | Self::User1 | Self::User2 | Self::User3 => {
                [128, 128, 128]
            }
        }
    }

    /// Diskriminante, wie sie in Script-Variablen als `i64` liegt.
    ///
    /// Discriminant stored in script variables as `i64`.
    pub fn as_i64(self) -> i64 {
        self as u8 as i64
    }
}

impl TryFrom<i64> for TrafficLightPhase {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Red1),
            1 => Ok(Self::Red2),
            2 => Ok(Self::Red3),
            3 => Ok(Self::YellowA1),
            4 => Ok(Self::YellowA2),
            5 => Ok(Self::YellowA3),
            6 => Ok(Self::Green1),
            7 => Ok(Self::Green2),
            8 => Ok(Self::Green3),
            9 => Ok(Self::YellowB1),
            10 => Ok(Self::YellowB2),
            11 => Ok(Self::YellowB3),
            12 => Ok(Self::OffPrimary),
            13 => Ok(Self::OffSecondary),
            14 => Ok(Self::User1),
            15 => Ok(Self::User2),
            16 => Ok(Self::User3),
            other => Err(other),
        }
    }
}
