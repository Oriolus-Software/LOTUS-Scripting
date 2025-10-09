use crate::time::{delta, delta_f64};

pub use lotus_shared::math::*;

pub fn exponential_approach(old_value: f32, exponent: f32, target: f32) -> f32 {
    let factor = 1.0 - (-delta() * exponent).exp();
    old_value + factor * (target - old_value)
}

pub fn exponential_approach_64(old_value: f64, exponent: f64, target: f64) -> f64 {
    let factor = 1.0 - (-delta_f64() * exponent).exp();
    old_value + factor * (target - old_value)
}
