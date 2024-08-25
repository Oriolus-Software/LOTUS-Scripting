use lotus_script_sys::FfiObject;
pub use lotus_shared::message::*;

mod types;
pub use types::*;

#[doc(hidden)]
pub fn get() -> Vec<Message> {
    let messages = FfiObject::from_packed(unsafe { lotus_script_sys::messages::take() });

    messages.deserialize()
}
