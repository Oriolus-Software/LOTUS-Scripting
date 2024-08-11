pub use lotus_bindgen_macros::lotus_bindgen;

use message::Message;
use serde::{de::DeserializeOwned, Serialize};

pub mod action;
pub mod content;
pub mod event;
pub(crate) mod ffi;
pub mod gizmos;
pub mod log;
pub mod macros;
pub mod message;
pub mod public_vars;
pub mod rand;
#[doc(hidden)]
pub mod settings;
pub mod var;
pub mod prelude {
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
    unsafe { ffi::time::delta_f64() as f32 }
}

/// Get the current delta time in seconds.
pub fn delta_f64() -> f64 {
    unsafe { ffi::time::delta_f64() }
}

/// Returns true if the object the script is attached to is remote controlled.
pub fn is_rc() -> bool {
    unsafe { ffi::is_rc() }
}

#[doc(hidden)]
pub struct FfiObject {
    data: Box<[u8]>,
}

impl FfiObject {
    pub fn new<T: Serialize>(value: &T) -> Self {
        let data = rmp_serde::to_vec_named(value)
            .expect("Failed to serialize value")
            .into_boxed_slice();

        Self { data }
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> T {
        rmp_serde::from_slice(&self.data).expect("Failed to deserialize value")
    }

    pub fn packed(&self) -> u64 {
        let ptr = self.data.as_ptr() as u32;
        let len = self.data.len() as u32;

        let mut packed = [0u8; 8];
        packed[..4].copy_from_slice(&ptr.to_be_bytes());
        packed[4..].copy_from_slice(&len.to_be_bytes());

        u64::from_be_bytes(packed)
    }

    pub fn packed_forget(self) -> u64 {
        let packed = self.packed();
        std::mem::forget(self);
        packed
    }

    pub fn from_packed(packed: u64) -> Self {
        let packed = packed.to_be_bytes();
        let ptr = u32::from_be_bytes(packed[..4].try_into().unwrap());
        let len = u32::from_be_bytes(packed[4..].try_into().unwrap());

        let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
        Self { data: data.into() }
    }
}
