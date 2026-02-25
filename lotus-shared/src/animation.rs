#[cfg(feature = "bevy")]
use bevy::prelude::Component;
use glam::Vec3;
#[cfg(feature = "ffi")]
use lotus_script_sys::FfiObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum AnimationError {
    #[error("animation not found")]
    AnimationNotFound = 65536,
    #[error("unknown error")]
    Unknown = 0,
}

impl From<u32> for AnimationError {
    fn from(value: u32) -> Self {
        match value {
            65536 => AnimationError::AnimationNotFound,
            _ => AnimationError::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct AccelerationVelocity {
    pub linear_velocity: Vec3,
    pub linear_acceleration: Vec3,
    pub angular_velocity: Vec3,
    pub angular_acceleration: Vec3,
}

impl AccelerationVelocity {
    pub fn relative_to_parent(
        self,
        parent: &AccelerationVelocity,
        position_relative_to_parent: Vec3,
    ) -> Self {
        Self {
            angular_velocity: parent.angular_velocity + self.angular_velocity,
            angular_acceleration: parent.angular_acceleration
                + self.angular_acceleration
                + parent.angular_velocity.cross(self.angular_velocity),
            linear_velocity: parent.linear_velocity
                + parent.angular_velocity.cross(position_relative_to_parent)
                + self.linear_velocity,

            linear_acceleration: parent.linear_acceleration
                + parent
                    .angular_acceleration
                    .cross(position_relative_to_parent)
                + parent
                    .angular_velocity
                    .cross(parent.angular_velocity.cross(position_relative_to_parent))
                + (2.0 * parent.angular_velocity).cross(self.linear_velocity)
                + self.linear_acceleration,
        }
    }
}

#[cfg(feature = "ffi")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Animation {
    index: usize,
}

#[cfg(feature = "ffi")]
impl Animation {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn get(name: &str) -> Result<Self, AnimationError> {
        let name = FfiObject::new(&name);

        match unsafe { lotus_script_sys::animation::get_animation_index(name.packed()) } {
            65536 => Err(AnimationError::AnimationNotFound),
            index => Ok(Self {
                index: index as usize,
            }),
        }
    }

    pub fn get_animation_global_acceleration_velocity(self) -> AccelerationVelocity {
        let state = unsafe {
            lotus_script_sys::animation::get_animation_global_acceleration_velocity(
                self.index as i32,
            )
        };

        FfiObject::from_packed(state).deserialize()
    }
}
