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

impl RailQuality {
    #[cfg(feature = "ffi")]
    /// Provides the quality of the rails under the given axis.
    pub fn get(bogie: usize, axle: usize) -> Result<RailQuality, VehicleError> {
        let quality = unsafe { lotus_script_sys::vehicle::rail_quality(bogie as u32, axle as u32) };
        RailQuality::try_from(quality)
    }
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

impl SurfaceType {
    #[cfg(feature = "ffi")]
    /// Provides the type of the surface under the given axis.
    pub fn get(bogie: usize, axle: usize) -> Result<SurfaceType, VehicleError> {
        let surface_type =
            unsafe { lotus_script_sys::vehicle::surface_type(bogie as u32, axle as u32) };
        SurfaceType::try_from(surface_type)
    }
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
