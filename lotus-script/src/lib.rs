pub use lotus_bindgen_macros::lotus_bindgen;

use message::Message;

pub mod action;
pub mod assets;
pub mod content;
#[doc(hidden)]
pub mod event;
pub mod gizmos;
pub mod graphics;
pub mod log;
pub mod macros;
pub mod math;
pub mod message;
pub mod public_vars;
pub mod rand;
#[doc(hidden)]
pub mod settings;
pub mod var;
pub mod prelude {
    pub use crate::action;
    pub use crate::log;
    pub use crate::message::{Message, MessageType};
}

pub mod input {
    pub use lotus_shared::input::*;
}

pub trait Script {
    fn init(&mut self) {}
    fn actions() -> Vec<action::RegisterAction> {
        Default::default()
    }

    fn tick(&mut self) {}

    #[allow(unused_variables)]
    fn on_message(&mut self, msg: Message) {}
}

/// Get the current delta time in seconds.
pub fn delta() -> f32 {
    unsafe { lotus_script_sys::time::delta_f64() as f32 }
}

/// Get the current delta time in seconds.
pub fn delta_f64() -> f64 {
    unsafe { lotus_script_sys::time::delta_f64() }
}

/// Returns true if the object the script is attached to is remote controlled.
pub fn is_rc() -> bool {
    unsafe { lotus_script_sys::is_rc() }
}
