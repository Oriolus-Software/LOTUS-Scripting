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

/// Returns the height of the lowest contact wire above the pantograph position.
pub fn pantograph_height(pantograph: usize) -> Result<f32, VehicleError> {
    let height = unsafe { lotus_script_sys::vehicle::pantograph_height(pantograph as u32) };
    if height.is_nan() {
        Err(VehicleError::VehicleNotFound)
    } else if height == f32::INFINITY {
        Err(VehicleError::PantographNotFound)
    } else {
        Ok(height)
    }
}

/// The voltage of the contact wire above the pantograph. The value is normalized, i.e. 1.0 means that the target voltage is present.
/// However, the script itself must check whether the pantograph is touching the contact wire.
pub fn pantograph_voltage(pantograph: usize) -> Result<f32, VehicleError> {
    let voltage = unsafe { lotus_script_sys::vehicle::pantograph_voltage(pantograph as u32) };
    if voltage.is_nan() {
        Err(VehicleError::VehicleNotFound)
    } else if voltage == f32::INFINITY {
        Err(VehicleError::PantographNotFound)
    } else {
        Ok(voltage)
    }
}
