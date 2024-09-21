use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Serialize for Vec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.x, self.y).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
        Ok(Self { x, y })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl Serialize for UVec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.x, self.y).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for UVec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let (x, y) = <(u32, u32)>::deserialize(deserializer)?;
        Ok(Self { x, y })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.x, self.y, self.z).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let (x, y, z) = <(f32, f32, f32)>::deserialize(deserializer)?;
        Ok(Self { x, y, z })
    }
}

#[cfg(feature = "bevy")]
mod _bevy {
    use super::*;

    impl From<bevy::math::UVec2> for UVec2 {
        fn from(value: bevy::math::UVec2) -> Self {
            Self {
                x: value.x,
                y: value.y,
            }
        }
    }

    impl From<UVec2> for bevy::math::UVec2 {
        fn from(value: UVec2) -> Self {
            Self {
                x: value.x,
                y: value.y,
            }
        }
    }

    impl From<bevy::math::Vec3> for Vec3 {
        fn from(value: bevy::math::Vec3) -> Self {
            Self {
                x: value.x,
                y: value.y,
                z: value.z,
            }
        }
    }

    impl From<Vec3> for bevy::math::Vec3 {
        fn from(value: Vec3) -> Self {
            Self {
                x: value.x,
                y: value.y,
                z: value.z,
            }
        }
    }
}
