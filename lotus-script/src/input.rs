use lotus_script_sys::FfiObject;
use lotus_shared::math::Vec2;

pub use lotus_shared::input::*;

/// Get the delta of the mouse since the last frame.
/// TODO: Specify the units.
pub fn mouse_delta() -> Vec2 {
    let delta = unsafe { lotus_script_sys::input::mouse_delta() };
    FfiObject::from_packed(delta).deserialize()
}

pub fn mouse_position() -> Vec2 {
    let delta = unsafe { lotus_script_sys::input::mouse_position() };
    FfiObject::from_packed(delta).deserialize()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MouseSteeringMode {
    #[default]
    Inactive,
    Active,
    ActiveBoost,
}

impl From<u32> for MouseSteeringMode {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Active,
            2 => Self::ActiveBoost,
            _ => Self::Inactive,
        }
    }
}

pub fn mouse_steering_mode() -> MouseSteeringMode {
    let active = unsafe { lotus_script_sys::input::mouse_steering_mode() };
    active.into()
}
