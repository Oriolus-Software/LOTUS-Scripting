//! Handle messages between scripts or from the engine.
//! See [Message] and [MessageType] for more information.
use std::borrow::Cow;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Represents a message that can be sent between scripts or from the engine.
///
/// # Example
/// ```no_run
/// # use serde::{Deserialize, Serialize};
/// # use lotus_script::prelude::*;
/// # use lotus_shared::message::{Message, MessageType};
/// #[derive(Serialize, Deserialize)]
/// struct TestMessage {
///     value: i32,
/// }
///
/// impl MessageType for TestMessage {
///     const MESSAGE_META: MessageMeta = MessageMeta::new("test", "message");
/// }
///
/// fn handle(message: &Message) {
///     message.handle::<TestMessage>(|m| {
///        log::info!("Received message: {}", m.value);
///        Ok(())
///     }).expect("message handle failed");
/// }
/// # handle(&Message::new(TestMessage { value: 42 }));
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    meta: MessageMeta,
    value: serde_json::Value,
}

/// Represents the metadata for a message type.
///
/// The combination of `namespace` and `identifier` should be unique for each message type.
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
    pub fn new<T: MessageType>(value: T) -> Self {
        Self {
            meta: T::MESSAGE_META.clone(),
            value: serde_json::to_value(value).unwrap(),
        }
    }

    /// Returns the message type metadata.
    pub fn meta(&self) -> &MessageMeta {
        &self.meta
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

    /// Sends the message to the given target.
    #[cfg(feature = "ffi")]
    pub fn send(&self, target: MessageTarget) {
        let this = lotus_script_sys::FfiObject::new(self);
        let target = lotus_script_sys::FfiObject::new(&target);

        unsafe { lotus_script_sys::messages::send(target.packed(), this.packed()) }
    }
}

/// Represents a message target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageTarget {
    /// The script itself.
    Myself,
    /// The child script at the given index.
    ChildByIndex(usize),
    /// All scripts of this vehicle.
    Broadcast,
    /// All scripts of this vehicle except this one.
    BroadcastExceptSelf,
    /// The parent script.
    Parent,
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestMessage {
        value: i32,
    }

    impl MessageType for TestMessage {
        const MESSAGE_META: MessageMeta = MessageMeta::new("test", "message", None);
    }

    #[test]
    fn test_message() {
        let message = Message::new(TestMessage { value: 42 });
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
