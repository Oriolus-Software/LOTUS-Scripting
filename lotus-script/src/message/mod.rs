pub use lotus_shared::message::*;

mod types;
pub use types::*;

use crate::FfiObject;

#[doc(hidden)]
pub fn get() -> Vec<Message> {
    let messages = FfiObject::from_packed(unsafe { crate::ffi::messages::get() });

    messages.deserialize()
}
