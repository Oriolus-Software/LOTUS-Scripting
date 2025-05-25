pub use lotus_shared::vehicle::*;

/// Gets the curvature of the track under the given axis.
/// The curvature is the reciprocal of the radius (1/R), which has the advantage that the value does not tend to infinity in a straight line, but tends to 0.
/// The values are very small due to this calculation: Even a radius of only 100m leads to a value of 0.01, larger radii lead to even smaller values.
/// Positive = right, negative = left.
pub fn inverse_radius(bogie: usize, axle: usize) -> Result<f32, VehicleError> {
    let inverse_radius =
        unsafe { lotus_script_sys::vehicle::inverse_radius(bogie as u32, axle as u32) };
    if inverse_radius.is_nan() {
        Err(VehicleError::VehicleNotFound)
    } else if inverse_radius == f32::NEG_INFINITY {
        Err(VehicleError::BogieNotFound)
    } else if inverse_radius == f32::INFINITY {
        Err(VehicleError::AxleNotFound)
    } else {
        Ok(inverse_radius)
    }
}

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

/// Sets the traction force in newton.
/// This is the torque applied to the axle, already converted to the force acting on the running surface. This means that as long as the wheel does not slip or spin, this value is equal to the force exerted by the wheel on the rail.
/// This force acts independently of the direction of travel. If it acts in the opposite direction to the travel, the vehicle will be braked, but it cannot hold the vehicle stationary.
pub fn traction_force_newton(bogie: usize, axle: usize, value: f32) {
    unsafe { lotus_script_sys::vehicle::traction_force_newton(bogie as u32, axle as u32, value) };
}

/// Sets the brake force in newton.
/// This is the torque applied to the axle, already converted to the force acting on the running surface.
/// This means that as long as the wheel does not slip or spin, this value is equal to the force exerted by the wheel on the rail.
/// The difference to "traction_force_newton" is that brake_force_newton is always positive and always acts in the opposite direction to the travel.
/// This means that brake_force_newton can also hold the vehicle stationary like a disc brake.
pub fn brake_force_newton(bogie: usize, axle: usize, value: f32) {
    unsafe { lotus_script_sys::vehicle::brake_force_newton(bogie as u32, axle as u32, value) };
}

/// Sets the rail brake force at the given bogie.
/// The rail brake consists of electromagnets that are set against the rail.
/// They then slide over the rail with high friction, which allows the vehicle to be braked much more strongly:
/// While the normal wheel brake only has the axle load available to build up a frictional grip with the rail,
/// the rail brake can exert much higher frictional forces and thus braking forces,
/// even relatively independent of the rail condition (moisture and dirt are effectively "ground off").
pub fn rail_brake_force_newton(bogie: usize, value: f32) {
    unsafe { lotus_script_sys::vehicle::rail_brake_force_newton(bogie as u32, value) };
}
