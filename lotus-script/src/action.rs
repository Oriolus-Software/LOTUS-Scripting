use lotus_script_sys::FfiObject;
pub use lotus_shared::action::*;
use lotus_shared::input::ActionState;

/// Get the current state of an action. If the action is not registered, it will return `ActionState::None`.
pub fn state(action: &str) -> ActionState {
    let action = FfiObject::new(&action);
    let state = unsafe { lotus_script_sys::action::state(action.packed()) };

    FfiObject::from_packed(state).deserialize()
}

#[doc(hidden)]
pub fn register_many(actions: &[RegisterAction]) {
    for action in actions {
        let action = FfiObject::new(&action);
        unsafe {
            lotus_script_sys::action::register(action.packed());
        }
    }
}
