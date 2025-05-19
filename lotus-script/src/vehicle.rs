use lotus_shared::vehicles::{RailQuality, SurfaceType};

/// Provides the quality of the rails under the given axis.
/// If the result is “None”, then the quality is still unknown at compile time.
pub fn rail_quality(bogie: u8, axle: u8) -> Option<RailQuality> {
    let quality = unsafe { lotus_script_sys::vehicle::rail_quality(bogie as u64, axle as u64) };
    RailQuality::try_from(quality as u8).ok()
}

/// Provides the type of the surface under the given axis.
/// If the result is “None”, then the surface type is still unknown at compile time.
pub fn surface_type(bogie: u8, axle: u8) -> Option<SurfaceType> {
    let surface_type =
        unsafe { lotus_script_sys::vehicle::surface_type(bogie as u64, axle as u64) };
    SurfaceType::try_from(surface_type as u8).ok()
}
