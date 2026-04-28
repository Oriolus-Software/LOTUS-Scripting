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

pub fn mouse_steering_active() -> bool {
    unsafe { lotus_script_sys::input::mouse_steering_active() }
}
