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
        pub fn game_time() -> i64;
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
        pub fn create(options: u64) -> u32;
        pub fn add_action(texture: u32, options: u64);
        pub fn get_pixel(texture: u32, x: u32, y: u32) -> u32;
        pub fn apply_to(texture: u32, name: u64);
        pub fn flush_actions(texture: u32) -> u32;
        pub fn dispose(texture: u32);
        pub fn fetch_drawable_texture_properties() -> u64;
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
        pub fn get_bool(name: u64) -> i32;
        pub fn set_bool(name: u64, value: i32);
        pub fn get_content_id(name: u64) -> u64;
        pub fn set_content_id(name: u64, value: u64);
    }
}

pub mod rand {
    #[link(wasm_import_module = "rand")]
    extern "C" {
        pub fn f64() -> f64;
        /// Generate a random u64 in the range `min` to `max` inclusive.
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

pub mod input {
    #[link(wasm_import_module = "input")]
    extern "C" {
        pub fn mouse_delta() -> u64;
    }
}

pub mod font {
    #[link(wasm_import_module = "font")]
    extern "C" {
        pub fn bitmap_font_properties(font: u64) -> u64;

        /// Returns: -1 if the font is not loaded.
        /// Returns: >0 is the width of the text.
        pub fn text_len(font: u64, text: u64, letter_spacing: i32) -> i32;
    }
}

pub mod vehicle {
    #[link(wasm_import_module = "vehicle")]
    extern "C" {
        pub fn bogie_is_valid(bogie: u32) -> u32;
        pub fn axle_is_valid(bogie: u32, axle: u32) -> u32;
        pub fn pantograph_is_valid(end: u32) -> u32;
        pub fn is_coupled(coupling: u32) -> u32;
        pub fn rail_quality(bogie: u32, axle: u32) -> u32;
        pub fn surface_type(bogie: u32, axle: u32) -> u32;
        pub fn inverse_radius(bogie: u32, axle: u32) -> f32;
        pub fn velocity_vs_ground() -> f32;
        pub fn acceleration_vs_ground() -> f32;
        pub fn pantograph_height(pantograph: u32) -> f32;
        pub fn pantograph_voltage(pantograph: u32) -> f32;
        pub fn set_traction_force_newton(bogie: u32, axle: u32, value: f32);
        pub fn set_brake_force_newton(bogie: u32, axle: u32, value: f32);
        pub fn set_rail_brake_force_newton(bogie: u32, value: f32);
    }
}

pub mod module {
    #[link(wasm_import_module = "module")]
    extern "C" {
        pub fn module_slot_cockpit_index() -> i32;
        pub fn module_slot_index_in_class_group() -> i32;
        pub fn module_slot_index() -> i32;
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

enum FfiObjectData {
    Boxed(Box<[u8]>),
    Raw(*mut u8, usize),
}

impl FfiObjectData {
    fn as_slice(&self) -> &[u8] {
        match self {
            Self::Boxed(data) => data,
            Self::Raw(ptr, len) => unsafe { std::slice::from_raw_parts(*ptr, *len) },
        }
    }
}

impl Drop for FfiObject {
    fn drop(&mut self) {
        match self.data {
            FfiObjectData::Boxed(_) => {}
            FfiObjectData::Raw(ptr, len) => unsafe {
                std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align(len, 8).unwrap())
            },
        }
    }
}

pub struct FfiObject {
    data: FfiObjectData,
}

impl FfiObject {
    pub fn new<T: Serialize>(value: &T) -> Self {
        let data = rmp_serde::to_vec_named(value)
            .expect("Failed to serialize value")
            .into_boxed_slice();

        Self {
            data: FfiObjectData::Boxed(data),
        }
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> T {
        rmp_serde::from_slice(self.data.as_slice()).expect("Failed to deserialize value")
    }

    pub fn packed(&self) -> u64 {
        let ptr = self.data.as_slice().as_ptr() as u32;
        let len = self.data.as_slice().len() as u32;

        let mut packed = [0u8; 8];
        packed[..4].copy_from_slice(&ptr.to_be_bytes());
        packed[4..].copy_from_slice(&len.to_be_bytes());

        u64::from_be_bytes(packed)
    }

    pub fn from_packed(packed: u64) -> Self {
        let packed = packed.to_be_bytes();
        let ptr = u32::from_be_bytes(packed[..4].try_into().unwrap());
        let len = u32::from_be_bytes(packed[4..].try_into().unwrap());

        Self {
            data: FfiObjectData::Raw(ptr as *mut u8, len as usize),
        }
    }
}
