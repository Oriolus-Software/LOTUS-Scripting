use lotus_shared::message::MessageMeta;
use serde::{Deserialize, Serialize};

use super::MessageType;

/// Represents an event triggered by a sensor in the system.
///
/// A trigger event occurs when an object enters or leaves a sensor's detection area.
/// Each sensor has a unique index and can detect both entry and exit events.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerEvent {
    /// Unique identifier for the trigger event
    pub id: String,
    /// Index of the sensor that generated this event
    pub sensor_index: i32,
    /// The type of trigger (Enter or Leave)
    pub kind: TriggerKind,
}

impl TriggerEvent {
    /// Returns true if this is an entry event
    pub fn is_enter(&self) -> bool {
        self.kind.is_enter()
    }

    /// Returns true if this is a leave event
    pub fn is_leave(&self) -> bool {
        self.kind.is_leave()
    }
}

impl MessageType for TriggerEvent {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "trigger_event", None);
}

/// Represents the type of trigger event that occurred.
#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerKind {
    /// Indicates an object entered the sensor's detection area
    Enter,
    /// Indicates an object left the sensor's detection area
    Leave,
}

impl TriggerKind {
    /// Returns true if this trigger represents an entry event
    pub fn is_enter(&self) -> bool {
        matches!(self, Self::Enter)
    }

    /// Returns true if this trigger represents a leave event
    pub fn is_leave(&self) -> bool {
        matches!(self, Self::Leave)
    }
}

/// Represents a button press or release event in the cockpit.
///
/// Button events capture the state changes of buttons in different
/// cockpit positions, tracking whether they are pressed or released.
#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonEvent {
    /// Unique identifier for the button event
    pub id: String,
    /// Current state of the button (true = pressed, false = released)
    pub value: bool,
    /// Index identifying the cockpit position of this button
    pub cockpit_index: u8,
}

impl MessageType for ButtonEvent {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "button_event", None);
}

/// Represents the state of the battery switch.
///
/// A simple wrapper around a boolean value indicating whether
/// the battery is switched on (true) or off (false).
#[derive(Debug, Serialize, Deserialize)]
pub struct BatterySwitch(pub bool);

impl MessageType for BatterySwitch {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "battery_switch", None);
}
