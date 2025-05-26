pub use lotus_shared::vehicle::*;

/// Returns the velocity over ground, measured along the vehicle.
/// Any spinning wheels etc. are therefore not taken into account.
pub fn velocity_vs_ground() -> f32 {
    unsafe { lotus_script_sys::vehicle::velocity_vs_ground() }
}

/// Returns the acceleration over ground, measured along the vehicle.
/// Any spinning wheels etc. are therefore not taken into account.
pub fn acceleration_vs_ground() -> f32 {
    unsafe { lotus_script_sys::vehicle::acceleration_vs_ground() }
}
