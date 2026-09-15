//! Nachrichten zwischen Scripts oder von der Engine verarbeiten.
//! Siehe [Message] und [MessageType] für weitere Informationen.
//!
//! Handle messages between scripts or from the engine.
//! See [Message] and [MessageType] for more information.
use std::borrow::Cow;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Nachricht, die zwischen Scripts oder von der Engine gesendet werden kann.
///
/// Represents a message that can be sent between scripts or from the engine.
///
/// # Example
/// ```no_run
/// # use serde::{Deserialize, Serialize};
/// # use lotus_shared::message::{Message, MessageType};
/// # use lotus_shared::message_type;
///
/// // Define a message type, has to implement Serialize and Deserialize
/// #[derive(Serialize, Deserialize)]
/// struct TestMessage {
///     value: i32,
/// }
///
/// // Register the message type
/// message_type!(TestMessage, "test", "message");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    meta: MessageMeta,
    #[cfg_attr(feature = "engine", serde(default))]
    source: MessageSource,
    value: serde_json::Value,
}

/// Metadaten für einen Nachrichtentyp.
///
/// Die Kombination aus `namespace` und `identifier` sollte für jeden Nachrichtentyp global eindeutig sein.
///
/// Represents the metadata for a message type.
///
/// The combination of `namespace` and `identifier` should be globally unique for each message type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageMeta {
    /// Namespace des Nachrichtentyps.
    ///
    /// The namespace of the message type.
    pub namespace: Cow<'static, str>,
    /// Identifikator des Nachrichtentyps.
    ///
    /// The identifier of the message type.
    pub identifier: Cow<'static, str>,
    /// Bus, über den die Nachricht gesendet werden soll.
    ///
    /// The bus the message should be sent on.
    pub bus: Option<Cow<'static, str>>,
}

impl MessageMeta {
    /// Erstellt neue Nachrichten-Metadaten.
    ///
    /// Creates a new message meta.
    pub const fn new(
        namespace: &'static str,
        identifier: &'static str,
        bus: Option<&'static str>,
    ) -> Self {
        Self {
            namespace: Cow::Borrowed(namespace),
            identifier: Cow::Borrowed(identifier),
            bus: match bus {
                Some(bus) => Some(Cow::Borrowed(bus)),
                None => None,
            },
        }
    }
}

/// Nachrichtentyp, der zwischen Scripts oder von der Engine gesendet werden kann.
///
/// Die Konstante [`MessageType::MESSAGE_META`] sollte global eindeutige Metadaten für den Nachrichtentyp liefern.
///
/// Represents a message type that can be sent between scripts or from the engine.
///
/// The [`MessageType::MESSAGE_META`] constant should return a globally unique message meta for the message type.
pub trait MessageType: Serialize + DeserializeOwned {
    /// Metadaten für den Nachrichtentyp.
    ///
    /// The metadata for the message type.
    const MESSAGE_META: MessageMeta;
}

/// Quelle einer Nachricht.
///
/// Represents the source of a message.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageSource {
    /// Kupplung, über die die Nachricht von einem anderen Fahrzeug kommt, falls zutreffend.
    ///
    /// If the message is coming from another vehicle across couplings, this will be Some.
    pub coupling: Option<Coupling>,
    /// Modul-Slot-Index des sendenden Moduls, falls zutreffend.
    ///
    /// If the message is coming from a module, these will be Some.
    pub module_slot_index: Option<u16>,
    /// Cockpit-Index des sendenden Modul-Slots, falls vorhanden.
    ///
    /// Cockpit index of the sending module slot, if applicable.
    pub module_slot_cockpit_index: Option<u8>,
}

impl MessageSource {
    /// Gibt `true` zurück, wenn die Nachricht vom vorderen Fahrzeug kommt.
    ///
    /// Returns `true` if the message is coming from the vehicle in front.
    pub fn is_front(&self) -> bool {
        matches!(self.coupling, Some(Coupling::Front))
    }

    /// Gibt `true` zurück, wenn die Nachricht vom hinteren Fahrzeug kommt.
    ///
    /// Returns `true` if the message is coming from the vehicle in rear.
    pub fn is_rear(&self) -> bool {
        matches!(self.coupling, Some(Coupling::Rear))
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! message_type {
    ($type:ty, $namespace:expr, $identifier:expr, $bus:expr) => {
        impl $crate::message::MessageType for $type {
            const MESSAGE_META: $crate::message::MessageMeta =
                $crate::message::MessageMeta::new($namespace, $identifier, Some($bus));
        }
    };
    ($type:ty, $namespace:expr, $identifier:expr) => {
        impl $crate::message::MessageType for $type {
            const MESSAGE_META: $crate::message::MessageMeta =
                $crate::message::MessageMeta::new($namespace, $identifier, None);
        }
    };
}

#[doc(inline)]
pub use message_type;

/// Fehler, wenn das Deserialisieren einer Nachrichtennutzlast fehlschlägt.
///
/// Error returned when deserializing a message payload fails.
#[derive(Debug, thiserror::Error)]
pub enum MessageValueError {
    /// Der Nachrichtentyp entspricht nicht dem angeforderten Typ.
    ///
    /// The message type does not match the requested type.
    #[error("invalid message type")]
    InvalidType,
    #[error("{0}")]
    Serialization(SerializationError),
}

/// Serialisierungsfehler beim Lesen oder Schreiben von Nachrichtendaten.
///
/// Serialization failure while reading or writing message data.
#[derive(Debug, thiserror::Error)]
#[error("serialization error: {0}")]
pub struct SerializationError(String);

/// Fehler, der von [`Message::handle`] zurückgegeben wird.
///
/// Error returned by [`Message::handle`].
#[derive(Debug, thiserror::Error)]
pub enum MessageHandleError {
    /// Fehler beim Deserialisieren der Nachrichtennutzlast.
    ///
    /// Failure while deserializing the message payload.
    #[error("{0}")]
    Serialization(SerializationError),
    /// Die Handler-Funktion hat einen Fehler zurückgegeben.
    ///
    /// The handler function returned an error.
    #[error("handler error: {0}")]
    Handler(Box<dyn std::error::Error>),
}

impl Message {
    /// Erstellt eine neue Nachricht mit dem angegebenen Wert.
    ///
    /// Creates a new message with the given value.
    pub fn new<T: MessageType>(value: &T) -> Self {
        Self {
            meta: T::MESSAGE_META.clone(),
            source: MessageSource::default(),
            value: serde_json::to_value(value).unwrap(),
        }
    }

    /// Gibt die Metadaten des Nachrichtentyps zurück.
    ///
    /// Returns the message type metadata.
    pub fn meta(&self) -> &MessageMeta {
        &self.meta
    }

    /// Gibt die Quelle der Nachricht zurück.
    ///
    /// Returns the source of the message.
    pub fn source(&self) -> &MessageSource {
        &self.source
    }

    /// Gibt den Nachrichtenwert als angegebenen Typ zurück.
    /// Liefert einen [`MessageValueError`], wenn der Typ nicht passt.
    ///
    /// Returns the message value as the given type. Returns a [MessageValueError] if the message has a different type.
    pub fn value<T: MessageType>(&self) -> Result<T, MessageValueError> {
        if self.meta != T::MESSAGE_META {
            return Err(MessageValueError::InvalidType);
        }

        serde_json::from_value(self.value.clone())
            .map_err(|e| MessageValueError::Serialization(SerializationError(e.to_string())))
    }

    /// Gibt `true` zurück, wenn die Nachricht den angegebenen Typ hat.
    ///
    /// Returns `true` if the message has the given type.
    pub fn has_type<T: MessageType>(&self) -> bool {
        self.meta == T::MESSAGE_META
    }

    /// Verarbeitet die Nachricht mit der angegebenen Handler-Funktion.
    /// Liefert `Ok(true)`, wenn verarbeitet, `Ok(false)` bei Typabweichung,
    /// oder `Err` bei Deserialisierungs- bzw. Handlerfehler.
    ///
    /// Die Handler-Funktion sollte `Ok(())` zurückgeben, wenn die Nachricht erfolgreich verarbeitet wurde.
    ///
    /// Handle the message with the given handler function.
    /// Returns `Ok(true)` if the message was handled, `Ok(false)` if the message has a different type,
    /// or `Err` if the message could not be deserialized or the handler function returned an error.
    ///
    /// The handler function should return `Ok(())` if the message was handled successfully.
    pub fn handle<T: MessageType>(
        &self,
        f: impl FnOnce(T) -> Result<(), Box<dyn std::error::Error>>,
    ) -> Result<bool, MessageHandleError> {
        match self.value::<T>() {
            Ok(v) => f(v).map_err(MessageHandleError::Handler).map(|_| true),
            Err(MessageValueError::InvalidType) => Ok(false),
            Err(MessageValueError::Serialization(e)) => Err(MessageHandleError::Serialization(e)),
        }
    }

    #[cfg(feature = "engine")]
    /// Gibt eine Kopie dieser Nachricht mit aktualisierter Quelle zurück.
    ///
    /// Returns a copy of this message with an updated source.
    pub fn with_source(&self, source: MessageSource) -> Self {
        Self {
            meta: self.meta.clone(),
            source,
            value: self.value.clone(),
        }
    }
}

/// Wandelt einen Wert in einen oder mehrere [`MessageTarget`]-Empfänger um.
///
/// Converts a value into one or more [`MessageTarget`] recipients.
pub trait IntoMessageTargets {
    fn into_message_targets(self) -> impl IntoIterator<Item = MessageTarget>;
}

impl IntoMessageTargets for MessageTarget {
    fn into_message_targets(self) -> impl IntoIterator<Item = MessageTarget> {
        [self]
    }
}

impl<T> IntoMessageTargets for T
where
    T: IntoIterator<Item = MessageTarget>,
{
    fn into_message_targets(self) -> impl IntoIterator<Item = MessageTarget> {
        self
    }
}

/// Sendet die Nachricht an die angegebenen Ziele.
///
/// Sends the message to the given targets.
///
/// # Example
/// ```no_run
/// # #[cfg(target_arch = "wasm32")]
/// # {
/// # use lotus_shared::message::{Message, MessageTarget, send_message};
/// # use serde::{Deserialize, Serialize};
/// # use lotus_shared::message_type;
/// # #[derive(Serialize, Deserialize)]
/// # struct TestMessage { value: i32 };
/// # message_type!(TestMessage, "test", "message");
/// // Send a message with only a single target
/// send_message(&TestMessage { value: 42 }, MessageTarget::Myself);
/// // Send a message to multiple targets
/// send_message(&TestMessage { value: 42 }, [MessageTarget::Myself, MessageTarget::ModuleSlot(0)]);
/// # }
/// ```
#[cfg(feature = "ffi")]
pub fn send_message<T: MessageType>(message: &T, targets: impl IntoMessageTargets) {
    let message = Message::new(message);
    let this = lotus_script_sys::FfiObject::new(&message);
    let targets = targets
        .into_message_targets()
        .into_iter()
        .collect::<Vec<_>>();
    let targets = lotus_script_sys::FfiObject::new(&targets);

    unsafe { lotus_script_sys::messages::send(targets.packed(), this.packed()) }
}

/// Ziel einer Nachricht.
///
/// Represents a message target.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MessageTarget {
    /// Das Script selbst.
    ///
    /// The script itself.
    Myself,
    /// Das Child-Script am angegebenen Index in der Child-Script-Liste.
    ///
    /// The child script at the given index in the child-script list.
    #[deprecated(note = "use `MessageTarget::ModuleSlot` for vehicle module slots")]
    #[doc(hidden)]
    ChildByIndex(usize),
    /// Das Modul im Fahrzeug-Modulslot mit dem angegebenen Index (`module_slot_index`).
    ///
    /// The module in the vehicle module slot with the given index (`module_slot_index`).
    ModuleSlot(usize),
    /// An alle Module im Cockpit mit dem angegebenen Index.
    ///
    /// To all modules in the cockpit with the given index.
    Cockpit(u8),
    /// Broadcast an Scripts gemäß dem angegebenen Umfang.
    ///
    /// Broadcast to scripts based on the specified scope.
    Broadcast {
        /// Ob gekuppelte Fahrzeuge einbezogen werden.
        ///
        /// Whether to include coupled vehicles.
        across_couplings: bool,
        /// Ob das sendende Script einbezogen wird.
        ///
        /// Whether to include the sending script.
        include_self: bool,
    },
    /// Senden an eine bestimmte Kupplung.
    ///
    /// Send to a specific coupling.
    AcrossCoupling {
        /// Kupplung, an die gesendet wird.
        ///
        /// The coupling to send to.
        coupling: Coupling,
        /// Ob die Nachricht zur nächsten Kupplung weitergeleitet wird.
        ///
        /// Whether to cascade the message to the next coupling.
        cascade: bool,
    },
    /// Das übergeordnete Script.
    ///
    /// The parent script.
    Parent,
}

impl MessageTarget {
    /// Hilfsfunktion für Broadcast-Ziele ohne das eigene Script.
    ///
    /// Helper to create a broadcast target that excludes self
    pub fn broadcast_except_self(across_couplings: bool) -> Self {
        Self::Broadcast {
            across_couplings,
            include_self: false,
        }
    }

    /// Sendet an alle Scripts in der Zugbildung, einschließlich des eigenen.
    ///
    /// Broadcasts to all scripts in the train composition, including self.
    pub fn broadcast_all() -> Self {
        Self::Broadcast {
            across_couplings: true,
            include_self: true,
        }
    }
}

/// Kupplungsrichtung zwischen Fahrzeugen in einem Zug.
///
/// Coupling direction between vehicles in a train.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Coupling {
    /// Kupplung zum vorderen Fahrzeug.
    ///
    /// The coupling to the front vehicle.
    Front,
    /// Kupplung zum hinteren Fahrzeug.
    ///
    /// The coupling to the rear vehicle.
    Rear,
}

impl Coupling {
    #[cfg(feature = "ffi")]
    /// Öffnet den angegebenen Bus.
    ///
    /// Opens the given bus.
    pub fn open_bus(&self, bus: &str) {
        let bus = lotus_script_sys::FfiObject::new(&bus);
        unsafe { lotus_script_sys::vehicle::open_bus(*self as u32, bus.packed()) };
    }

    #[cfg(feature = "ffi")]
    /// Schließt den angegebenen Bus.
    ///
    /// Closes the given bus.
    pub fn close_bus(&self, bus: &str) {
        let bus = lotus_script_sys::FfiObject::new(&bus);
        unsafe { lotus_script_sys::vehicle::close_bus(*self as u32, bus.packed()) };
    }

    #[cfg(feature = "ffi")]
    /// Gibt `true` zurück, wenn der angegebene Bus geöffnet ist.
    ///
    /// Returns `true` if the given bus is open.
    pub fn is_open(&self, bus: &str) -> bool {
        let bus = lotus_script_sys::FfiObject::new(&bus);
        unsafe { lotus_script_sys::vehicle::is_bus_open(*self as u32, bus.packed()) == 1 }
    }

    #[cfg(feature = "ffi")]
    /// Gibt `true` zurück, wenn an dieser Kupplung ein Fahrzeug gekoppelt ist.
    ///
    /// Returns `true` if a vehicle is coupled at this coupling.
    pub fn is_coupled(&self) -> bool {
        unsafe { lotus_script_sys::vehicle::is_coupled(*self as u32) == 1 }
    }
}

impl From<u32> for Coupling {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Front,
            1 => Self::Rear,
            _ => panic!("invalid coupling value: {}", value),
        }
    }
}

impl From<usize> for Coupling {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Front,
            1 => Self::Rear,
            _ => panic!("invalid coupling value: {}", value),
        }
    }
}

impl From<Coupling> for usize {
    fn from(value: Coupling) -> Self {
        match value {
            Coupling::Front => 0,
            Coupling::Rear => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestMessage {
        value: i32,
    }

    message_type!(TestMessage, "test", "message", "ibis");

    #[test]
    fn test_message() {
        let message = Message::new(&TestMessage { value: 42 });
        assert_eq!(message.meta(), &TestMessage::MESSAGE_META);

        let value = message.value::<TestMessage>().unwrap();

        assert_eq!(value, TestMessage { value: 42 });

        message
            .handle::<TestMessage>(|m| {
                assert_eq!(m.value, 42);
                Ok(())
            })
            .expect("message handle failed");
    }
}
