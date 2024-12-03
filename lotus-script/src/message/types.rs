use lotus_shared::message::MessageMeta;
use serde::{Deserialize, Serialize};

use super::MessageType;

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerEvent {
    pub id: String,
    pub sensor_index: i32,
    pub kind: TriggerKind,
}

impl TriggerEvent {
    pub fn is_enter(&self) -> bool {
        self.kind.is_enter()
    }

    pub fn is_leave(&self) -> bool {
        self.kind.is_leave()
    }
}

impl MessageType for TriggerEvent {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "trigger_event");
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerKind {
    Enter,
    Leave,
}

impl TriggerKind {
    pub fn is_enter(&self) -> bool {
        matches!(self, Self::Enter)
    }

    pub fn is_leave(&self) -> bool {
        matches!(self, Self::Leave)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonEvent {
    pub id: String,
    pub value: bool,
    pub cockpit_index: u8,
}

impl MessageType for ButtonEvent {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "button_event");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatterySwitch(pub bool);

impl MessageType for BatterySwitch {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "battery_switch");
}
