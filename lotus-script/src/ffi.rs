use lotus_bindgen_macros::lotus_bindgen;

use crate::FfiObject;

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

pub mod time {
    #[link(wasm_import_module = "time")]
    extern "C" {
        pub fn delta_f64() -> f64;
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
        pub fn get() -> u64;
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

impl FromFfi for crate::content::ContentId {
    type FfiType = u64;
    fn from_ffi(ffi: Self::FfiType) -> Self {
        FfiObject::from_packed(ffi).deserialize()
    }
}
