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
    /// Builds axle-local kinematics in LOTUS coordinates (X = lateral, Y = longitudinal, Z = vertical)
    /// and rotates them into the global simulation frame.
    pub fn from_rail_axle_local(
        longitudinal_velocity: f32,
        longitudinal_acceleration: f32,
        inv_radius: f32,
        elevation_derivation: f32,
        elevation_second_derivation: f32,
        axle_rotation: glam::Quat,
    ) -> Self {
        let v = longitudinal_velocity;
        let a_long = longitudinal_acceleration;
        let lateral_acceleration = v * v * inv_radius;
        let vertical_acceleration =
            elevation_second_derivation * v * v + elevation_derivation * a_long;

        let local = Self {
            linear_velocity: Vec3::new(0.0, v, elevation_derivation * v),
            linear_acceleration: Vec3::new(lateral_acceleration, a_long, vertical_acceleration),
            angular_velocity: Vec3::new(0.0, 0.0, v * inv_radius),
            angular_acceleration: Vec3::new(0.0, 0.0, a_long * inv_radius),
        };

        local.transform_axes_to_global(axle_rotation)
    }

    /// Rotates linear/angular velocity and acceleration vectors into the global frame.
    pub fn transform_axes_to_global(self, rotation: glam::Quat) -> Self {
        Self {
            linear_velocity: rotation * self.linear_velocity,
            linear_acceleration: rotation * self.linear_acceleration,
            angular_velocity: rotation * self.angular_velocity,
            angular_acceleration: rotation * self.angular_acceleration,
        }
    }

    /// Rotates linear/angular velocity and acceleration vectors into a local frame.
    pub fn transform_axes_to_local(self, rotation: glam::Quat) -> Self {
        let inv = rotation.inverse();
        Self {
            linear_velocity: inv * self.linear_velocity,
            linear_acceleration: inv * self.linear_acceleration,
            angular_velocity: inv * self.angular_velocity,
            angular_acceleration: inv * self.angular_acceleration,
        }
    }

    pub fn average(self, other: Self) -> Self {
        Self {
            linear_velocity: 0.5 * (self.linear_velocity + other.linear_velocity),
            linear_acceleration: 0.5 * (self.linear_acceleration + other.linear_acceleration),
            angular_velocity: 0.5 * (self.angular_velocity + other.angular_velocity),
            angular_acceleration: 0.5 * (self.angular_acceleration + other.angular_acceleration),
        }
    }

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

    /// Linear and angular acceleration at a fixed point `local_offset` (animation-unit local frame),
    /// expressed in the animation-unit local frame.
    ///
    /// Includes Euler (`α × r`) and centripetal (`ω × (ω × r)`) contributions for rigid-body motion.
    pub fn acceleration_at_local_point(self, local_offset: Vec3) -> LocalPointAcceleration {
        LocalPointAcceleration {
            linear_acceleration: self.linear_acceleration
                + self.angular_acceleration.cross(local_offset)
                + self.angular_velocity.cross(self.angular_velocity.cross(local_offset)),
            angular_acceleration: self.angular_acceleration,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct LocalPointAcceleration {
    pub linear_acceleration: Vec3,
    pub angular_acceleration: Vec3,
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
