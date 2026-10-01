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

/// Gibt die Kraft zurück, die die Räder in die Lenkung einspeisen.
///
/// Der Wert stammt aus dem letzten Physikschritt und liegt in der normierten
/// Einheit des Lenkkraftspeichers (Trägheit 1). Ein positiver Wert dreht die
/// Lenkung zum rechten Anschlag, ein negativer zum linken. Der eigene Sollwert
/// aus [`set_road_steering_force`] ist nicht enthalten; die Engine multipliziert
/// ihn mit 10, bevor sie ihn in denselben Speicher schreibt.
///
/// Ohne Fahrzeugkontext ist der Wert `NaN`, sonst `0`, wenn keine Radkraft anliegt.
///
/// Returns the force the wheels feed into the steering.
///
/// The value comes from the last physics step, in the normalized steering-force
/// unit (inertia 1). Positive turns the steering toward the right stop, negative
/// toward the left. The command from [`set_road_steering_force`] is not included;
/// the engine multiplies that command by 10 before adding it to the same accumulator.
///
/// Without a vehicle context the value is `NaN`, otherwise `0` when no wheel force is applied.
pub fn road_steering_wheel_force() -> f32 {
    unsafe { lotus_script_sys::vehicle::road_steering_wheel_force() }
}

/// Setzt den Lenkwinkel der ersten Achse direkt, ohne Feder, Dämpfer oder äußere Kräfte.
///
/// `-1.0` ist der linke Anschlag, `+1.0` der rechte. Der Wert gilt nur für den aktuellen Tick.
///
/// Sets the first-axle steering angle directly, bypassing spring, damper and external forces.
///
/// `-1.0` is the left stop, `+1.0` the right stop. The value applies only to the current tick.
pub fn set_road_steering_direct(value: f32) {
    unsafe { lotus_script_sys::vehicle::set_road_steering_direct(value) }
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
