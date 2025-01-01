use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    input::{ActionState, KeyCode},
    message::{MessageMeta, MessageType},
};

/// Describes an action that can be registered with the engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAction {
    pub id: String,
    pub default_key: KeyCode,
}

/// Describes an event that is sent when an action is triggered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionEvent {
    pub name: String,
    pub state: ActionState,
}

impl MessageType for ActionEvent {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "action_event", None);
}

/// Describes the kind of action that was triggered.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionKind {
    Pressed = 1,
    Released = 2,
}
