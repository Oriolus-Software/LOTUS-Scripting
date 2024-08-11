use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{input::KeyCode, message::MessageType};

/// Describes an action that can be registered with the engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAction {
    pub id: String,
    pub default_key: KeyCode,
}

/// Describes an event that is sent when an action is triggered.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionEvent {
    name: String,
    kind: ActionKind,
}

impl MessageType for ActionEvent {
    fn id() -> &'static str {
        "builtin:action_event"
    }
}

/// Describes the kind of action that was triggered.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionKind {
    Pressed = 1,
    Released = 2,
}
