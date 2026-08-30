//! Eingebaute Nachrichtentypen für Trigger, Tasten und Zugbildung.
//!
//! Built-in message types for triggers, buttons, and train composition.

use lotus_shared::message::message_type;
use serde::{Deserialize, Serialize};

/// Ereignis, das durch einen Sensor ausgelöst wird.
///
/// Represents an event triggered by a sensor in the system.
///
/// Ein Trigger-Ereignis tritt auf, wenn ein Objekt den Erfassungsbereich eines Sensors betritt oder verlässt.
///
/// A trigger event occurs when an object enters or leaves a sensor's detection area.
/// Each sensor has a unique index and can detect both entry and exit events.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerEvent {
    /// Eindeutige Kennung des Trigger-Ereignisses.
    ///
    /// Unique identifier for the trigger event.
    pub id: String,
    /// Index des Sensors, der das Ereignis erzeugt hat.
    ///
    /// Index of the sensor that generated this event.
    pub sensor_index: i32,
    /// Art des Triggers (Betreten oder Verlassen).
    ///
    /// The type of trigger (Enter or Leave).
    pub kind: TriggerKind,
}

impl TriggerEvent {
    /// Gibt `true` zurück, wenn es sich um ein Betreten-Ereignis handelt.
    ///
    /// Returns `true` if this is an entry event.
    pub fn is_enter(&self) -> bool {
        self.kind.is_enter()
    }

    /// Gibt `true` zurück, wenn es sich um ein Verlassen-Ereignis handelt.
    ///
    /// Returns `true` if this is a leave event.
    pub fn is_leave(&self) -> bool {
        self.kind.is_leave()
    }
}

message_type!(TriggerEvent, "builtin", "trigger_event");

/// Art eines Trigger-Ereignisses.
///
/// Represents the type of trigger event that occurred.
#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerKind {
    /// Objekt hat den Erfassungsbereich des Sensors betreten.
    ///
    /// An object entered the sensor's detection area.
    Enter,
    /// Objekt hat den Erfassungsbereich des Sensors verlassen.
    ///
    /// An object left the sensor's detection area.
    Leave,
}

impl TriggerKind {
    /// Gibt `true` zurück, wenn der Trigger ein Betreten-Ereignis darstellt.
    ///
    /// Returns `true` if this trigger represents an entry event.
    pub fn is_enter(&self) -> bool {
        matches!(self, Self::Enter)
    }

    /// Gibt `true` zurück, wenn der Trigger ein Verlassen-Ereignis darstellt.
    ///
    /// Returns `true` if this trigger represents a leave event.
    pub fn is_leave(&self) -> bool {
        matches!(self, Self::Leave)
    }
}

/// Tastendruck- oder Loslass-Ereignis im Cockpit.
///
/// Represents a button press or release event in the cockpit.
///
/// Erfasst Zustandsänderungen von Tasten an verschiedenen Cockpit-Positionen.
///
/// Button events capture state changes of buttons in different cockpit positions,
/// tracking whether they are pressed or released.
#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonEvent {
    /// Eindeutige Kennung des Tastenereignisses.
    ///
    /// Unique identifier for the button event.
    pub id: String,
    /// Aktueller Tastenzustand (`true` = gedrückt, `false` = losgelassen).
    ///
    /// Current state of the button (`true` = pressed, `false` = released).
    pub value: bool,
    /// Cockpit-Index der Taste.
    ///
    /// Cockpit index of this button.
    pub cockpit_index: u8,
}

message_type!(ButtonEvent, "builtin", "button_event");

/// Zustand des Batterieschalters.
///
/// Represents the state of the battery switch.
///
/// Einfacher Boolean-Wrapper: `true` = Batterie ein, `false` = Batterie aus.
///
/// Simple boolean wrapper: `true` = battery on, `false` = battery off.
#[derive(Debug, Serialize, Deserialize)]
pub struct BatterySwitch(pub bool);

message_type!(BatterySwitch, "builtin", "battery_switch");

pub use lotus_shared::vehicle::{
    TrainConfiguration, TrainConfigurationChanged, TrainVehicleConfiguration,
};
