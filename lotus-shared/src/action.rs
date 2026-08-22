//! Eingabeaktionen, die bei der Engine registriert werden, und zugehörige Nachrichtentypen.
//!
//! Input actions registered with the engine and related message types.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    input::{ActionState, KeyCode},
    message::{MessageMeta, MessageType},
};

/// Beschreibt eine Aktion, die bei der Engine registriert werden kann.
///
/// Describes an action that can be registered with the engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAction {
    /// Eindeutige Aktions-ID für Scripts und das Content-Tool.
    ///
    /// Unique action identifier used by scripts and the content tool.
    pub id: String,
    /// Standard-Tastenbelegung bei der Registrierung der Aktion.
    ///
    /// Default key binding when the action is registered.
    pub default_key: KeyCode,
}

impl RegisterAction {
    /// Erstellt einen neuen Registrierungseintrag für eine Aktion.
    ///
    /// Creates a new action registration entry.
    pub fn new(id: String, default_key: KeyCode) -> Self {
        Self { id, default_key }
    }
}

impl<T: Into<String>> From<(T, KeyCode)> for RegisterAction {
    fn from((id, default_key): (T, KeyCode)) -> Self {
        Self::new(id.into(), default_key)
    }
}

/// Builder zum Sammeln von [`RegisterAction`]-Einträgen.
///
/// Builder for collecting [`RegisterAction`] entries.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActionsBuilder(Vec<RegisterAction>);

impl ActionsBuilder {
    /// Erstellt einen leeren Builder.
    ///
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Fügt eine Aktion mit der angegebenen ID und Standardtaste hinzu.
    ///
    /// Adds an action with the given id and default key.
    pub fn push(mut self, id: impl Into<String>, default_key: KeyCode) -> Self {
        self.0.push(RegisterAction::new(id.into(), default_key));
        self
    }

    /// Gibt die gesammelten Aktionsregistrierungen zurück.
    ///
    /// Returns the collected action registrations.
    pub fn build(self) -> Vec<RegisterAction> {
        self.0
    }
}

/// Beschreibt ein Ereignis, das ausgelöst wird, wenn eine Aktion betätigt wird.
///
/// Describes an event that is sent when an action is triggered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionEvent {
    /// Name der ausgelösten Aktion.
    ///
    /// Name of the triggered action.
    pub name: String,
    /// Aktueller Eingabezustand der Aktion.
    ///
    /// Current input state of the action.
    pub state: ActionState,
}

impl MessageType for ActionEvent {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "action_event", None);
}

/// Beschreibt die Art der ausgelösten Aktion.
///
/// Describes the kind of action that was triggered.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionKind {
    /// Die Aktion wurde gedrückt.
    ///
    /// The action was pressed.
    Pressed = 1,
    /// Die Aktion wurde losgelassen.
    ///
    /// The action was released.
    Released = 2,
}
