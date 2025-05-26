use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, thiserror::Error)]
pub enum VehicleError {
    #[error("vehicle not found")]
    VehicleNotFound = 256,
    #[error("bogie not found")]
    BogieNotFound = 512,
    #[error("axle not found")]
    AxleNotFound = 1024,
    #[error("pantograph not found")]
    PantographNotFound = 2048,
    #[error("unknown error")]
    Unknown = 0,
}

impl From<u32> for VehicleError {
    fn from(value: u32) -> Self {
        match value {
            256 => VehicleError::VehicleNotFound,
            512 => VehicleError::BogieNotFound,
            1024 => VehicleError::AxleNotFound,
            2048 => VehicleError::PantographNotFound,
            _ => VehicleError::Unknown,
        }
    }
}

#[cfg(feature = "ffi")]
#[derive(Copy, Clone)]
pub struct Bogie {
    index: usize,
}

#[cfg(feature = "ffi")]
impl Bogie {
    pub fn get(index: usize) -> Result<Self, VehicleError> {
        match unsafe { lotus_script_sys::vehicle::bogie_is_valid(index as u32) } {
            0 => Ok(Self { index }),
            e => Err(e.into()),
        }
    }

    /// Sets the rail brake force at the given bogie.
    /// The rail brake consists of electromagnets that are set against the rail.
    /// They then slide over the rail with high friction, which allows the vehicle to be braked much more strongly:
    /// While the normal wheel brake only has the axle load available to build up a frictional grip with the rail,
    /// the rail brake can exert much higher frictional forces and thus braking forces,
    /// even relatively independent of the rail condition (moisture and dirt are effectively "ground off").
    pub fn set_rail_brake_force_newton(self, value: f32) {
        unsafe { lotus_script_sys::vehicle::set_rail_brake_force_newton(self.index as u32, value) };
    }
}

#[cfg(feature = "ffi")]
#[derive(Copy, Clone)]
pub struct Axle {
    bogie_index: usize,
    axle_index: usize,
}

#[cfg(feature = "ffi")]
impl Axle {
    pub fn get(bogie_index: usize, axle_index: usize) -> Result<Self, VehicleError> {
        match unsafe {
            lotus_script_sys::vehicle::axle_is_valid(bogie_index as u32, axle_index as u32)
        } {
            0 => Ok(Self {
                bogie_index,
                axle_index,
            }),
            e => Err(e.into()),
        }
    }

    pub fn velocity_var_name(self) -> String {
        format!("v_Axle_mps_{}_{}", self.bogie_index, self.axle_index)
    }

    pub fn bogie(self) -> Bogie {
        Bogie {
            index: self.bogie_index,
        }
    }

    /// Gets the curvature of the track under the given axis.
    /// The curvature is the reciprocal of the radius (1/R), which has the advantage that the value does not tend to infinity in a straight line, but tends to 0.
    /// The values are very small due to this calculation: Even a radius of only 100m leads to a value of 0.01, larger radii lead to even smaller values.
    /// Positive = right, negative = left.
    pub fn inverse_radius(self) -> f32 {
        let inverse_radius = unsafe {
            lotus_script_sys::vehicle::inverse_radius(
                self.bogie_index as u32,
                self.axle_index as u32,
            )
        };
        assert!(!inverse_radius.is_nan());
        assert_ne!(inverse_radius, f32::NEG_INFINITY);
        assert_ne!(inverse_radius, f32::INFINITY);
        inverse_radius
    }

    /// Provides the type of the surface under the given axis.
    pub fn surface_type(self) -> SurfaceType {
        let surface_type = unsafe {
            lotus_script_sys::vehicle::surface_type(self.bogie_index as u32, self.axle_index as u32)
        };
        SurfaceType::try_from(surface_type).unwrap()
    }

    /// Provides the quality of the rails under the given axis.
    pub fn rail_quality(self) -> RailQuality {
        let quality = unsafe {
            lotus_script_sys::vehicle::rail_quality(self.bogie_index as u32, self.axle_index as u32)
        };
        RailQuality::try_from(quality).unwrap()
    }

    /// Sets the traction force in newton.
    /// This is the torque applied to the axle, already converted to the force acting on the running surface. This means that as long as the wheel does not slip or spin, this value is equal to the force exerted by the wheel on the rail.
    /// This force acts independently of the direction of travel. If it acts in the opposite direction to the travel, the vehicle will be braked, but it cannot hold the vehicle stationary.
    pub fn set_traction_force_newton(self, value: f32) {
        unsafe {
            lotus_script_sys::vehicle::set_traction_force_newton(
                self.bogie_index as u32,
                self.axle_index as u32,
                value,
            )
        };
    }

    /// Sets the brake force in newton.
    /// This is the torque applied to the axle, already converted to the force acting on the running surface.
    /// This means that as long as the wheel does not slip or spin, this value is equal to the force exerted by the wheel on the rail.
    /// The difference to "traction_force_newton" is that brake_force_newton is always positive and always acts in the opposite direction to the travel.
    /// This means that brake_force_newton can also hold the vehicle stationary like a disc brake.
    pub fn set_brake_force_newton(self, value: f32) {
        unsafe {
            lotus_script_sys::vehicle::set_brake_force_newton(
                self.bogie_index as u32,
                self.axle_index as u32,
                value,
            )
        };
    }
}

/// Provides the quality of the rails under the given axis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RailQuality {
    Smooth = 0,
    Rough = 1,
    FroggySmooth = 2,
    FroggyRough = 3,
    FlatGroove = 4,
    HighSpeedSmooth = 5,
    SmoothDirt = 6,
    RoughDirt = 7,
}

impl TryFrom<u32> for RailQuality {
    type Error = VehicleError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RailQuality::Smooth),
            1 => Ok(RailQuality::Rough),
            2 => Ok(RailQuality::FroggySmooth),
            3 => Ok(RailQuality::FroggyRough),
            4 => Ok(RailQuality::FlatGroove),
            5 => Ok(RailQuality::HighSpeedSmooth),
            6 => Ok(RailQuality::SmoothDirt),
            7 => Ok(RailQuality::RoughDirt),
            value => Err(value.into()),
        }
    }
}

/// Type of the surface under the given axis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SurfaceType {
    Gravel = 0,
    Street = 1,
    Grass = 2,
}

impl TryFrom<u32> for SurfaceType {
    type Error = VehicleError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SurfaceType::Gravel),
            1 => Ok(SurfaceType::Street),
            2 => Ok(SurfaceType::Grass),
            value => Err(value.into()),
        }
    }
}
