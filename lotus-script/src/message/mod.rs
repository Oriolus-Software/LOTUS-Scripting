//! Nachrichten senden und empfangen (Re-Export aus `lotus_shared` plus Engine-Abholung).
//!
//! Sending and receiving messages (re-exported from `lotus_shared` plus engine polling).

use lotus_script_sys::FfiObject;
pub use lotus_shared::message::*;

mod types;
pub use types::*;

#[doc(hidden)]
pub fn get() -> Vec<Message> {
    let messages = FfiObject::from_packed(unsafe { lotus_script_sys::messages::take() });

    messages.deserialize()
}
