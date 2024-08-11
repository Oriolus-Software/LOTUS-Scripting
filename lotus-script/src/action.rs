use crate::{ffi, FfiObject};

pub use lotus_shared::action::*;
use lotus_shared::input::ActionState;

/// Get the current state of an action. If the action is not registered, it will return `ActionState::None`.
pub fn state(action: &str) -> ActionState {
    let action = FfiObject::new(&action);
    let state = unsafe { ffi::action::state(action.packed()) };

    FfiObject::from_packed(state).deserialize()
}

#[doc(hidden)]
pub fn register_many(actions: &[RegisterAction]) {
    for action in actions {
        let action = FfiObject::new(&action);
        unsafe {
            ffi::action::register(action.packed());
        }
    }
}
