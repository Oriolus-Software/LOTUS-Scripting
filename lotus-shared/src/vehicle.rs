use serde_repr::{Deserialize_repr, Serialize_repr};

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

impl TryFrom<u8> for RailQuality {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RailQuality::Smooth),
            1 => Ok(RailQuality::Rough),
            2 => Ok(RailQuality::FroggySmooth),
            3 => Ok(RailQuality::FroggyRough),
            4 => Ok(RailQuality::FlatGroove),
            5 => Ok(RailQuality::HighSpeedSmooth),
            6 => Ok(RailQuality::SmoothDirt),
            7 => Ok(RailQuality::RoughDirt),
            _ => Err(()),
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

impl TryFrom<u8> for SurfaceType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SurfaceType::Gravel),
            1 => Ok(SurfaceType::Street),
            2 => Ok(SurfaceType::Grass),
            _ => Err(()),
        }
    }
}
