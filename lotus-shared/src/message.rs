//! Handle messages between scripts or from the engine.
//! See [Message] and [MessageType] for more information.
use std::borrow::Cow;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
    source: MessageSource,
    value: serde_json::Value,
}

/// Represents the metadata for a message type.
///
/// The combination of `namespace` and `identifier` should be globally unique for each message type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageMeta {
    /// The namespace of the message type.
    pub namespace: Cow<'static, str>,
    /// The identifier of the message type.
    pub identifier: Cow<'static, str>,
    /// The bus the message should be sent on.
    pub bus: Option<Cow<'static, str>>,
}

impl MessageMeta {
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

/// Represents a message type that can be sent between scripts or from the engine.
/// The [MessageType::MESSAGE_META] constant should return a globally unique message meta for the message type.
pub trait MessageType: Serialize + DeserializeOwned {
    /// The metadata for the message type.
    const MESSAGE_META: MessageMeta;
}

/// Represents the source of a message.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageSource {
    /// If the message is coming from another vehicle across couplings, this will be Some.
    pub coupling: Option<Coupling>,
}

impl MessageSource {
    /// Returns `true` if the message is coming from the vehicle in front.
    pub fn is_front(&self) -> bool {
        matches!(self.coupling, Some(Coupling::Front))
    }

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

#[derive(Debug, thiserror::Error)]
pub enum MessageValueError {
    #[error("invalid message type")]
    InvalidType,
    #[error("{0}")]
    Serialization(SerializationError),
}

#[derive(Debug, thiserror::Error)]
#[error("serialization error: {0}")]
pub struct SerializationError(String);

#[derive(Debug, thiserror::Error)]
pub enum MessageHandleError {
    #[error("{0}")]
    Serialization(SerializationError),
    #[error("handler error: {0}")]
    Handler(Box<dyn std::error::Error>),
}

impl Message {
    /// Creates a new message with the given value.
    pub fn new<T: MessageType>(value: &T) -> Self {
        Self {
            meta: T::MESSAGE_META.clone(),
            source: MessageSource::default(),
            value: serde_json::to_value(value).unwrap(),
        }
    }

    /// Returns the message type metadata.
    pub fn meta(&self) -> &MessageMeta {
        &self.meta
    }

    /// Returns the source of the message.
    pub fn source(&self) -> &MessageSource {
        &self.source
    }

    /// Returns the message value as the given type. Returns a [MessageValueError] if the message has a different type.
    pub fn value<T: MessageType>(&self) -> Result<T, MessageValueError> {
        if self.meta != T::MESSAGE_META {
            return Err(MessageValueError::InvalidType);
        }

        serde_json::from_value(self.value.clone())
            .map_err(|e| MessageValueError::Serialization(SerializationError(e.to_string())))
    }

    /// Returns `true` if the message has the given type.
    pub fn has_type<T: MessageType>(&self) -> bool {
        self.meta == T::MESSAGE_META
    }

    /// Handle the message with the given handler function.
    /// Returns `Ok(true)` if the message was handled, `Ok(false)` if the message has a different type,
    /// or `Err` if the message could not be deserialized or the handler function returned an error.
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
    pub fn with_source(&self, source: MessageSource) -> Self {
        Self {
            meta: self.meta.clone(),
            source,
            value: self.value.clone(),
        }
    }
}

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

/// Sends the message to the given targets.
///
/// # Example
/// ```no_run
/// # use lotus_shared::message::{Message, MessageTarget, send_message};
/// # use serde::{Deserialize, Serialize};
/// # use lotus_shared::message_type;
/// # #[derive(Serialize, Deserialize)]
/// # struct TestMessage { value: i32 };
/// # message_type!(TestMessage, "test", "message");
/// // Send a message with only a single target
/// send_message(&TestMessage { value: 42 }, MessageTarget::Myself);
/// // Send a message to multiple targets
/// send_message(&TestMessage { value: 42 }, [MessageTarget::Myself, MessageTarget::ChildByIndex(0)]);
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

/// Represents a message target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageTarget {
    /// The script itself.
    Myself,
    /// The child script at the given index.
    ChildByIndex(usize),
    /// To all modules in the cockpit with the given index.
    Cockpit(u8),
    /// Broadcast to scripts based on the specified scope.
    Broadcast {
        /// Whether to include coupled vehicles.
        across_couplings: bool,
        /// Whether to include the sending script.
        include_self: bool,
    },
    /// Send to a specific coupling.
    AcrossCoupling {
        /// The coupling to send to.
        coupling: Coupling,
        /// Whether to cascade the message to the next coupling.
        cascade: bool,
    },
    /// The parent script.
    Parent,
}

impl MessageTarget {
    /// Helper to create a broadcast target that excludes self
    pub fn broadcast_except_self(across_couplings: bool) -> Self {
        Self::Broadcast {
            across_couplings,
            include_self: false,
        }
    }

    pub fn broadcast_all() -> Self {
        Self::Broadcast {
            across_couplings: true,
            include_self: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Coupling {
    /// The coupling to the front vehicle.
    Front,
    /// The coupling to the rear vehicle.
    Rear,
}

impl Coupling {
    /// Opens the given bus.
    pub fn open_bus(&self, _bus: &str) {
        todo!()
    }

    /// Closes the given bus.
    pub fn close_bus(&self, _bus: &str) {
        todo!()
    }

    /// Returns `true` if the given bus is open.
    pub fn is_open(&self, _bus: &str) -> bool {
        todo!()
    }

    #[cfg(feature = "ffi")]
    pub fn is_coupled(&self) -> bool {
        match self {
            Self::Front => unsafe { lotus_script_sys::vehicle::is_coupled(0) == 1 },
            Self::Rear => unsafe { lotus_script_sys::vehicle::is_coupled(1) == 1 },
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
