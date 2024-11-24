pub use lotus_bindgen_macros::lotus_bindgen;

use message::Message;

pub mod action;
pub mod assets;
pub mod content;
#[doc(hidden)]
pub mod event;
pub mod font;
pub mod gizmos;
pub mod graphics;
pub mod input;
pub mod log;
pub mod macros;
pub mod math;
pub mod message;
pub mod public_vars;
pub mod rand;
#[doc(hidden)]
pub mod settings;
pub mod time;
pub mod var;
pub mod prelude {
    pub use crate::action;
    pub use crate::log;
    pub use crate::message::{Message, MessageType};
}

pub trait Script {
    /// Initialize the script.
    fn init(&mut self) {}

    /// Register actions.
    fn actions() -> Vec<action::RegisterAction> {
        Default::default()
    }

    /// Tick the script.
    fn tick(&mut self) {}

    /// Handle a message.
    #[allow(unused_variables)]
    fn on_message(&mut self, msg: Message) {}
}

/// Returns true if the object the script is attached to is remote controlled.
pub fn is_rc() -> bool {
    unsafe { lotus_script_sys::is_rc() }
}
