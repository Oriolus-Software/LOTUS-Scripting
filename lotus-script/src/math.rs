use crate::time::delta;

pub use lotus_shared::math::*;

pub fn exponential_approach(old_value: f32, exponent: f32, target: f32) -> f32 {
    let factor = 1.0 - (-delta() * exponent).exp();
    old_value + factor * (target - old_value)
}
