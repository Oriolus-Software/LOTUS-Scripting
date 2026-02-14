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

/// If it is a road vehicle, you can set the steering angle of the first axle with this function.
/// The unit is degrees.
pub fn set_road_steering_force(force: f32) {
    unsafe { lotus_script_sys::vehicle::set_road_steering_force(force) }
}
