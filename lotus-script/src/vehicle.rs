//! Fahrzeugphysik und Zugbildung.
//!
//! Vehicle physics and train composition.

pub use lotus_shared::vehicle::*;

/// Gibt die Geschwindigkeit über Grund entlang der Fahrzeuglängsachse in m/s zurück.
/// Schlupfende Räder werden nicht berücksichtigt.
///
/// Returns velocity over ground along the vehicle in m/s.
/// Spinning wheels are not taken into account.
pub fn velocity_vs_ground() -> f32 {
    unsafe { lotus_script_sys::vehicle::velocity_vs_ground() }
}

/// Gibt die Beschleunigung über Grund entlang der Fahrzeuglängsachse in m/s² zurück.
/// Schlupfende Räder werden nicht berücksichtigt.
///
/// Returns acceleration over ground along the vehicle in m/s².
/// Spinning wheels are not taken into account.
pub fn acceleration_vs_ground() -> f32 {
    unsafe { lotus_script_sys::vehicle::acceleration_vs_ground() }
}

/// Setzt die Lenkkraft der ersten Achse bei Straßenfahrzeugen in Grad.
///
/// Sets the steering force of the first axle for road vehicles, in degrees.
pub fn set_road_steering_force(force: f32) {
    unsafe { lotus_script_sys::vehicle::set_road_steering_force(force) }
}

/// Manipuliert Federsteifigkeit und Dämpfung der Straßenlenkung.
///
/// Manipulates steering spring stiffness and damping for road vehicles.
pub fn set_road_steering_spring_damper_manipulation(values: RoadSteeringSpringDamperManipulator) {
    unsafe {
        lotus_script_sys::vehicle::set_road_steering_spring_damper_manipulation(
            values.stiffness_add,
            values.stiffness_mult,
            values.damping_add,
            values.damping_mult,
        )
    }
}
