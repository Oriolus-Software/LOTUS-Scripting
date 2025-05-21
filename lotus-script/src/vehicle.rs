pub use lotus_shared::vehicle::*;

/// Gets the inverse radius (1 / radius) under the given axis,
/// positive = right, negative = left.
pub fn inverse_radius(bogie: u8, axle: u8) -> Result<f32, VehicleError> {
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
