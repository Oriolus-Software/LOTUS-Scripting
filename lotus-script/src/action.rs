//! Abfrage registrierter Eingabeaktionen.
//!
//! Query registered input actions.

use lotus_script_sys::FfiObject;
pub use lotus_shared::action::*;
use lotus_shared::input::ActionState;

/// Gibt den aktuellen Zustand einer Aktion zurück.
/// Nicht registrierte Aktionen liefern [`crate::input::ActionStateKind::None`].
///
/// Returns the current state of an action.
/// Unregistered actions return [`crate::input::ActionStateKind::None`].
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
