pub use lotus_shared::vehicle::*;

/// Provides the quality of the rails under the given axis.
/// If the result is “None”, then the quality is still unknown at compile time.
pub fn railquality(bogie: u8, axle: u8) -> Option<RailQuality> {
    let quality = unsafe { lotus_script_sys::vehicle::railquality(bogie as u32, axle as u32) };
    RailQuality::try_from(quality as u8).ok()
}

/// Provides the type of the surface under the given axis.
/// If the result is “None”, then the surface type is still unknown at compile time.
pub fn surface_type(bogie: u8, axle: u8) -> Option<SurfaceType> {
    let surface_type = unsafe { lotus_script_sys::vehicle::surfacetype(bogie as u32, axle as u32) };
    SurfaceType::try_from(surface_type as u8).ok()
}

/// Gets the "inverse radius" (1 / radius) under the given axis,
/// positive = right, negative = left.
pub fn invradius(bogie: u8, axle: u8) -> f32 {
    unsafe { lotus_script_sys::vehicle::invradius(bogie as u32, axle as u32) }
}
