use lotus_bindgen_macros::lotus_bindgen;
use serde::{de::DeserializeOwned, Serialize};

#[no_mangle]
pub extern "C" fn allocate(size: u32) -> u32 {
    unsafe {
        std::alloc::alloc(std::alloc::Layout::from_size_align(size as usize, 8).unwrap()) as u32
    }
}

#[no_mangle]
pub extern "C" fn deallocate(ptr: u32, size: u32) {
    unsafe {
        std::alloc::dealloc(
            ptr as *mut u8,
            std::alloc::Layout::from_size_align(size as usize, 8).unwrap(),
        )
    }
}

extern "C" {
    #[lotus_bindgen]
    pub fn is_rc() -> bool;
}

pub mod assets {
    #[link(wasm_import_module = "assets")]
    extern "C" {
        pub fn preload(id: u64);
    }
}

pub mod time {
    #[link(wasm_import_module = "time")]
    extern "C" {
        pub fn delta_f64() -> f64;
        pub fn ticks_alive() -> u64;
    }
}

pub mod log {
    #[link(wasm_import_module = "log")]
    extern "C" {
        pub fn write(level: i32, message: u64);
    }
}

pub mod messages {
    #[link(wasm_import_module = "messages")]
    extern "C" {
        pub fn take() -> u64;
        pub fn send(target: u64, message: u64);
    }
}

pub mod textures {
    #[link(wasm_import_module = "textures")]
    extern "C" {
        pub fn create(options: u64) -> u64;
        pub fn add_action(texture: u64, options: u64);
        pub fn get_pixel(texture: u64, x: u32, y: u32) -> u32;
        pub fn apply_to(texture: u64, name: u64);
        pub fn flush_actions(texture: u64);
    }
}

pub mod var {
    #[link(wasm_import_module = "var")]
    extern "C" {
        pub fn get_i64(name: u64) -> i64;
        pub fn set_i64(name: u64, value: i64);
        pub fn get_f64(name: u64) -> f64;
        pub fn set_f64(name: u64, value: f64);
        pub fn get_string(name: u64) -> u64;
        pub fn set_string(name: u64, value: u64);
        pub fn get_bool(name: u64) -> bool;
        pub fn set_bool(name: u64, value: bool);
        pub fn get_content_id(name: u64) -> u64;
        pub fn set_content_id(name: u64, value: u64);
    }
}

pub mod rand {
    #[link(wasm_import_module = "rand")]
    extern "C" {
        pub fn f64() -> f64;
        /// Generate a random u64 in the range [min] to [max] inclusive.
        pub fn u64(min: u64, max: u64) -> u64;
        pub fn seed(seed: u64);
        pub fn random_seed();
    }
}

pub mod gizmo {
    #[link(wasm_import_module = "gizmo")]
    extern "C" {
        pub fn draw(gizmo: u64);
    }
}

pub mod action {
    #[link(wasm_import_module = "action")]
    extern "C" {
        pub fn register(action: u64);
        pub fn state(action: u64) -> u64;
    }
}

pub trait FromFfi {
    type FfiType;
    fn from_ffi(ffi: Self::FfiType) -> Self;
}

impl FromFfi for String {
    type FfiType = u64;
    fn from_ffi(ffi: Self::FfiType) -> Self {
        FfiObject::from_packed(ffi).deserialize()
    }
}

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