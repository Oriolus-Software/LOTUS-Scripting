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

impl RegisterAction {
    pub fn new(id: String, default_key: KeyCode) -> Self {
        Self { id, default_key }
    }
}

impl<T: Into<String>> From<(T, KeyCode)> for RegisterAction {
    fn from((id, default_key): (T, KeyCode)) -> Self {
        Self::new(id.into(), default_key)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActionsBuilder(Vec<RegisterAction>);

impl ActionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, id: impl Into<String>, default_key: KeyCode) -> Self {
        self.0.push(RegisterAction::new(id.into(), default_key));
        self
    }

    pub fn build(self) -> Vec<RegisterAction> {
        self.0
    }
}

/// Describes an event that is sent when an action is triggered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
