//! Fahrzeugphysik-Handles, Schienen-/Straßeneigenschaften und Zugbildungsnachrichten.
//!
//! Vehicle physics handles, rail/road properties, and train composition messages.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::message::{Coupling, MessageMeta, MessageType};

/// Fehler beim Auflösen von Fahrzeugkomponenten.
///
/// Errors returned when resolving vehicle components.
#[derive(Debug, thiserror::Error)]
pub enum VehicleError {
    #[error("vehicle not found")]
    VehicleNotFound = 256,
    #[error("bogie not found")]
    BogieNotFound = 512,
    #[error("axle not found")]
    AxleNotFound = 1024,
    #[error("coupling not found")]
    CouplingNotFound = 2048,
    #[error("pantograph not found")]
    PantographNotFound = 4096,
    #[error("road axle not found")]
    RoadAxleNotFound = 8192,
    #[error("road wheel not found")]
    RoadWheelNotFound = 16384,
    #[error("unknown error")]
    Unknown = 0,
}

impl From<u32> for VehicleError {
    fn from(value: u32) -> Self {
        match value {
            256 => VehicleError::VehicleNotFound,
            512 => VehicleError::BogieNotFound,
            1024 => VehicleError::AxleNotFound,
            2048 => VehicleError::CouplingNotFound,
            4096 => VehicleError::PantographNotFound,
            8192 => VehicleError::RoadAxleNotFound,
            16384 => VehicleError::RoadWheelNotFound,
            _ => VehicleError::Unknown,
        }
    }
}

#[cfg(feature = "ffi")]
/// Gibt `true` zurück, wenn das Fahrzeug zur Zugbildung invertiert gespawnt wurde.
///
/// Returns `true` if the vehicle was spawned inverted to the train.
pub fn spawned_inverted_to_train() -> bool {
    unsafe { lotus_script_sys::vehicle::spawned_inverted_to_train() == 1 }
}

/// Spawn-Snapshot eines Fahrzeugs innerhalb einer Zugbildung.
///
/// Initial spawn snapshot of one vehicle within a train composition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainVehicleConfiguration {
    /// Fahrzeugnummer bzw. Kennzeichnung als Text.
    ///
    /// Vehicle number or identifier string.
    pub number: String,
    /// Ob das Fahrzeug relativ zur Zugfahrtrichtung gedreht ist.
    ///
    /// Whether the vehicle is reversed relative to the train direction.
    pub reversed_to_train: bool,
}

/// Geordnete Liste der Fahrzeuge in einem Zug zum Spawn-Zeitpunkt.
///
/// Ordered list of vehicles in a train at spawn time.
pub type TrainConfiguration = Vec<TrainVehicleConfiguration>;

/// Ereignis bei Änderung der Zugbildung.
///
/// Hinweis: Werden Züge mit unterschiedlicher Fahrtrichtung gekuppelt, ist die neue Richtung nicht vorhersagbar.
/// In Fahrzeugen, deren Richtung beim Kuppeln invertiert wird, kehrt sich `reversed_to_train` um und die Indexreihenfolge dreht sich um.
///
/// Describes an event that is sent when the train configuration is changed.
///
/// Please note: When two trains with different directions are coupled,
/// the new direction cannot be predicted!
/// In the vehicles of the train whose direction is inverted when coupling,
/// "reversed_to_train" is inverted and the index order reverses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainConfigurationChanged {
    /// Entity-ID des betroffenen Fahrzeugs.
    ///
    /// Entity id of the affected vehicle.
    pub entity_id: u64,
    /// Ob dieses Fahrzeug relativ zum Zug gedreht ist.
    ///
    /// Whether this vehicle is reversed relative to the train.
    pub reversed_to_train: bool,
    /// Nullbasierter Index dieses Fahrzeugs im Zug.
    ///
    /// Zero-based index of this vehicle in the train.
    pub index_in_train: usize,
    /// Gesamtzahl der Fahrzeuge im Zug.
    ///
    /// Total number of vehicles in the train.
    pub train_vehicle_count: usize,
    /// Vollständige Zugbildung beim initialen Spawn; `None` bei späteren Updates (z. B. Script-Reload).
    ///
    /// Full train composition at initial spawn; `None` for later updates (e.g. script reload).
    #[serde(default)]
    pub train_configuration: Option<TrainConfiguration>,
}

impl MessageType for TrainConfigurationChanged {
    const MESSAGE_META: MessageMeta = MessageMeta::new("builtin", "vehicle_in_train_changed", None);
}

/// Berechnung der Fahrzeuganzahl vor oder hinter diesem Fahrzeug.
///
/// Calculation of the vehicle count in front or behind the vehicle
/// relative to the vehicle.
impl TrainConfigurationChanged {
    /// Gibt die Anzahl der Fahrzeuge in der angegebenen Richtung ab diesem Fahrzeug zurück.
    ///
    /// Returns the number of vehicles in the given direction from this vehicle.
    pub fn neighbour_vehicle_count(&self, coupling: Coupling) -> usize {
        if (coupling == Coupling::Rear) ^ self.reversed_to_train {
            self.train_vehicle_count - self.index_in_train - 1
        } else {
            self.index_in_train
        }
    }
}

/// Handle auf ein Schienenfahrzeug-Drehgestell.
///
/// Handle to a rail vehicle bogie.
#[cfg(feature = "ffi")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bogie {
    index: usize,
}

#[cfg(feature = "ffi")]
impl Bogie {
    /// Gibt das Drehgestell am angegebenen nullbasierten Index zurück.
    ///
    /// Returns the bogie at the given zero-based index.
    pub fn get(index: usize) -> Result<Self, VehicleError> {
        match unsafe { lotus_script_sys::vehicle::bogie_is_valid(index as u32) } {
            0 => Ok(Self { index }),
            e => Err(e.into()),
        }
    }

    /// Setzt die Schienenbremskraft am Drehgestell in Newton.
    ///
    /// Die Schienenbremse besteht aus Elektromagneten, die an die Schiene angelegt werden.
    /// Sie gleiten mit hoher Reibung über die Schiene und ermöglichen deutlich stärkere Bremsung:
    /// Während die normale Radbremse nur mit der Achslast Haftreibung aufbauen kann,
    /// kann die Schienenbremse wesentlich höhere Reib- und Bremskräfte ausüben,
    /// relativ unabhängig vom Schienenzustand (Feuchtigkeit und Schmutz werden faktisch „weggeschliffen“).
    ///
    /// Sets the rail brake force at the given bogie.
    /// The rail brake consists of electromagnets that are set against the rail.
    /// They then slide over the rail with high friction, which allows the vehicle to be braked much more strongly:
    /// While the normal wheel brake only has the axle load available to build up a frictional grip with the rail,
    /// the rail brake can exert much higher frictional forces and thus braking forces,
    /// even relatively independent of the rail condition (moisture and dirt are effectively "ground off").
    pub fn set_rail_brake_force_newton(self, value: f32) {
        unsafe { lotus_script_sys::vehicle::set_rail_brake_force_newton(self.index as u32, value) };
    }
}

/// Handle auf eine Schienenfahrzeug-Achse an einem Drehgestell.
///
/// Handle to a rail vehicle axle on a bogie.
#[cfg(feature = "ffi")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Axle {
    bogie_index: usize,
    axle_index: usize,
}

#[cfg(feature = "ffi")]
impl Axle {
    /// Gibt die Achse an den angegebenen Drehgestell- und Achsenindizes zurück.
    ///
    /// Returns the axle at the given bogie and axle indices.
    pub fn get(bogie_index: usize, axle_index: usize) -> Result<Self, VehicleError> {
        match unsafe {
            lotus_script_sys::vehicle::axle_is_valid(bogie_index as u32, axle_index as u32)
        } {
            0 => Ok(Self {
                bogie_index,
                axle_index,
            }),
            e => Err(e.into()),
        }
    }

    /// Gibt den Script-Variablennamen mit der Achsgeschwindigkeit in m/s zurück.
    ///
    /// Returns the script variable name containing this axle's velocity in m/s.
    pub fn velocity_var_name(self) -> String {
        format!("v_Axle_mps_{}_{}", self.bogie_index, self.axle_index)
    }

    /// Gibt das Drehgestell dieser Achse zurück.
    ///
    /// Returns the bogie this axle belongs to.
    pub fn bogie(self) -> Bogie {
        Bogie {
            index: self.bogie_index,
        }
    }

    /// Gibt den nullbasierten Achsenindex am Drehgestell zurück.
    ///
    /// Returns the zero-based axle index on the bogie.
    pub fn axle_index(self) -> usize {
        self.axle_index
    }

    /// Gibt den nullbasierten Drehgestellindex zurück.
    ///
    /// Returns the zero-based bogie index.
    pub fn bogie_index(self) -> usize {
        self.bogie_index
    }

    /// Gibt die Gleiskrümmung unter der Achse zurück (1/R).
    ///
    /// Die Krümmung ist der Kehrwert des Radius (1/R); in Geraden tendiert der Wert gegen 0 statt gegen Unendlichkeit.
    /// Die Werte sind daher sehr klein: Schon ein Radius von 100 m ergibt 0,01, größere Radien noch kleinere Werte.
    /// Positiv = rechts, negativ = links.
    ///
    /// Gets the curvature of the track under the given axis.
    /// The curvature is the reciprocal of the radius (1/R), which has the advantage that the value does not tend to infinity in a straight line, but tends to 0.
    /// The values are very small due to this calculation: Even a radius of only 100m leads to a value of 0.01, larger radii lead to even smaller values.
    /// Positive = right, negative = left.
    pub fn inverse_radius(self) -> f32 {
        let inverse_radius = unsafe {
            lotus_script_sys::vehicle::inverse_radius(
                self.bogie_index as u32,
                self.axle_index as u32,
            )
        };
        assert!(!inverse_radius.is_nan());
        assert_ne!(inverse_radius, f32::NEG_INFINITY);
        assert_ne!(inverse_radius, f32::INFINITY);
        inverse_radius
    }

    /// Gibt die Oberflächenart unter der Achse zurück.
    ///
    /// Provides the type of the surface under the given axis.
    pub fn surface_type(self) -> SurfaceType {
        let surface_type = unsafe {
            lotus_script_sys::vehicle::surface_type(self.bogie_index as u32, self.axle_index as u32)
        };
        SurfaceType::try_from(surface_type).unwrap()
    }

    /// Gibt die Schienenqualität unter der Achse zurück.
    ///
    /// Provides the quality of the rails under the given axis.
    pub fn rail_quality(self) -> RailQuality {
        let quality = unsafe {
            lotus_script_sys::vehicle::rail_quality(self.bogie_index as u32, self.axle_index as u32)
        };
        RailQuality::try_from(quality).unwrap()
    }

    /// Setzt die Traktionskraft in Newton.
    ///
    /// Dies ist das auf die Achse wirkende Drehmoment, bereits umgerechnet in die Kraft auf der Lauffläche.
    /// Solange das Rad nicht schlupft, entspricht der Wert der Kraft des Rades auf die Schiene.
    /// Die Kraft wirkt unabhängig von der Fahrtrichtung; entgegen der Fahrt bremst sie, hält das Fahrzeug aber nicht still.
    ///
    /// Sets the traction force in newton.
    /// This is the torque applied to the axle, already converted to the force acting on the running surface. This means that as long as the wheel does not slip or spin, this value is equal to the force exerted by the wheel on the rail.
    /// This force acts independently of the direction of travel. If it acts in the opposite direction to the travel, the vehicle will be braked, but it cannot hold the vehicle stationary.
    pub fn set_traction_force_newton(self, value: f32) {
        unsafe {
            lotus_script_sys::vehicle::set_traction_force_newton(
                self.bogie_index as u32,
                self.axle_index as u32,
                value,
            )
        };
    }

    /// Setzt die Bremskraft in Newton.
    ///
    /// Drehmoment auf der Achse, umgerechnet in Laufflächenkraft; ohne Schlupf entspricht das der Schienenkraft.
    /// Im Unterschied zu `set_traction_force_newton` ist die Bremskraft immer positiv und wirkt immer entgegen der Fahrtrichtung.
    /// Damit kann sie das Fahrzeug wie eine Scheibenbremse auch im Stand halten.
    ///
    /// Sets the brake force in newton.
    /// This is the torque applied to the axle, already converted to the force acting on the running surface.
    /// This means that as long as the wheel does not slip or spin, this value is equal to the force exerted by the wheel on the rail.
    /// The difference to "traction_force_newton" is that brake_force_newton is always positive and always acts in the opposite direction to the travel.
    /// This means that brake_force_newton can also hold the vehicle stationary like a disc brake.
    pub fn set_brake_force_newton(self, value: f32) {
        unsafe {
            lotus_script_sys::vehicle::set_brake_force_newton(
                self.bogie_index as u32,
                self.axle_index as u32,
                value,
            )
        };
    }
}

/// Handle auf eine Straßenfahrzeug-Achse.
///
/// Handle to a road vehicle axle.
#[cfg(feature = "ffi")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RoadAxle {
    index: usize,
}

#[cfg(feature = "ffi")]
impl RoadAxle {
    /// Gibt die Straßenachse am angegebenen nullbasierten Index zurück.
    ///
    /// Returns the road axle at the given zero-based index.
    pub fn get(index: usize) -> Result<Self, VehicleError> {
        match unsafe { lotus_script_sys::vehicle::road_axle_is_valid(index as u32) } {
            0 => Ok(Self { index }),
            e => Err(e.into()),
        }
    }
}

/// Handle auf ein Straßenfahrzeug-Rad an einer Achse.
///
/// Handle to a road vehicle wheel on an axle.
#[cfg(feature = "ffi")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RoadWheel {
    axle_index: usize,
    wheel_index: usize,
}

#[cfg(feature = "ffi")]
impl RoadWheel {
    /// Gibt das Rad an den angegebenen Achsen- und Radindizes zurück.
    ///
    /// Returns the wheel at the given axle and wheel indices.
    pub fn get(axle_index: usize, wheel_index: usize) -> Result<Self, VehicleError> {
        match unsafe {
            lotus_script_sys::vehicle::road_wheel_is_valid(axle_index as u32, wheel_index as u32)
        } {
            0 => Ok(Self {
                axle_index,
                wheel_index,
            }),
            e => Err(e.into()),
        }
    }

    /// Gibt den Script-Variablennamen mit der Radgeschwindigkeit in m/s zurück.
    ///
    /// Returns the script variable name containing this wheel's velocity in m/s.
    pub fn velocity_var_name(self) -> String {
        format!("v_wheel_mps_{}_{}", self.axle_index, self.wheel_index)
    }

    /// Gibt den nullbasierten Radindex an der Achse zurück.
    ///
    /// Returns the zero-based wheel index on the axle.
    pub fn wheel_index(self) -> usize {
        self.wheel_index
    }

    /// Gibt den nullbasierten Achsenindex zurück.
    ///
    /// Returns the zero-based axle index.
    pub fn axle_index(self) -> usize {
        self.axle_index
    }

    /// Setzt die Traktionskraft an der Lauffläche in Newton.
    ///
    /// Sets the traction force at the running surface in newton.
    pub fn set_traction_force_newton(self, value: f32) {
        unsafe {
            lotus_script_sys::vehicle::set_wheel_traction_force_newton(
                self.axle_index as u32,
                self.wheel_index as u32,
                value,
            )
        };
    }

    /// Setzt die Bremskraft an der Lauffläche in Newton.
    ///
    /// Sets the brake force at the running surface in newton.
    pub fn set_brake_force_newton(self, value: f32) {
        unsafe {
            lotus_script_sys::vehicle::set_wheel_brake_force_newton(
                self.axle_index as u32,
                self.wheel_index as u32,
                value,
            )
        };
    }

    /// Setzt den Faktor zur Manipulation der Federsteifigkeit.
    ///
    /// Sets the factor, which manipulates the spring stiffness.
    pub fn set_spring_factor(self, value: f32) {
        unsafe {
            lotus_script_sys::vehicle::set_wheel_spring_factor(
                self.axle_index as u32,
                self.wheel_index as u32,
                value,
            )
        };
    }
}

/// Handle auf einen Stromabnehmer am aktuellen Fahrzeug.
///
/// Handle to a pantograph on the current vehicle.
#[cfg(feature = "ffi")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pantograph {
    index: usize,
}

#[cfg(feature = "ffi")]
impl Pantograph {
    /// Gibt den Stromabnehmer am angegebenen nullbasierten Index zurück.
    ///
    /// Returns the pantograph at the given zero-based index.
    pub fn get(index: usize) -> Result<Self, VehicleError> {
        match unsafe { lotus_script_sys::vehicle::pantograph_is_valid(index as u32) } {
            0 => Ok(Self { index }),
            e => Err(e.into()),
        }
    }

    /// Gibt die Höhe des tiefsten Fahrdrads über der Stromabnehmer-Position zurück.
    ///
    /// Returns the height of the lowest contact wire above the pantograph position.
    pub fn height(self) -> f32 {
        let height = unsafe { lotus_script_sys::vehicle::pantograph_height(self.index as u32) };
        assert!(!height.is_nan());
        assert_ne!(height, f32::INFINITY);
        height
    }

    /// Spannung der Fahrleitung über dem Stromabnehmer (normalisiert; 1.0 = Sollspannung).
    /// Das Script muss selbst prüfen, ob der Stromabnehmer die Leitung berührt.
    ///
    /// The voltage of the contact wire above the pantograph. The value is normalized, i.e. 1.0 means that the target voltage is present.
    /// However, the script itself must check whether the pantograph is touching the contact wire.
    pub fn voltage(self) -> f32 {
        let voltage = unsafe { lotus_script_sys::vehicle::pantograph_voltage(self.index as u32) };
        assert!(!voltage.is_nan());
        assert_ne!(voltage, f32::INFINITY);
        voltage
    }
}

/// Beschreibt die Schienenqualität unter der angegebenen Achse.
///
/// Provides the quality of the rails under the given axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
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
    Deraileur = 8,
}

impl TryFrom<u32> for RailQuality {
    type Error = VehicleError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RailQuality::Smooth),
            1 => Ok(RailQuality::Rough),
            2 => Ok(RailQuality::FroggySmooth),
            3 => Ok(RailQuality::FroggyRough),
            4 => Ok(RailQuality::FlatGroove),
            5 => Ok(RailQuality::HighSpeedSmooth),
            6 => Ok(RailQuality::SmoothDirt),
            7 => Ok(RailQuality::RoughDirt),
            8 => Ok(RailQuality::Deraileur),
            value => Err(value.into()),
        }
    }
}

/// Art der Oberfläche unter der angegebenen Achse.
///
/// Type of the surface under the given axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SurfaceType {
    Gravel = 0,
    Street = 1,
    Grass = 2,
}

impl TryFrom<u32> for SurfaceType {
    type Error = VehicleError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SurfaceType::Gravel),
            1 => Ok(SurfaceType::Street),
            2 => Ok(SurfaceType::Grass),
            value => Err(value.into()),
        }
    }
}

/// Parameter zum Verändern von Feder- und Dämpferverhalten der Straßenlenkung.
///
/// Parameters for manipulating road steering spring and damper behavior.
#[derive(Clone, Copy)]
pub struct RoadSteeringSpringDamperManipulator {
    /// Steifigkeits-Additiv zum Standardwert.
    ///
    /// The stiffness is added to the default stiffness.
    pub stiffness_add: f32,
    /// Steifigkeits-Multiplikator auf den Standardwert.
    ///
    /// The stiffness is multiplied by the default stiffness.
    pub stiffness_mult: f32,
    /// Dämpfungs-Additiv zum Standardwert.
    ///
    /// The damping is added to the default damping.
    pub damping_add: f32,
    /// Dämpfungs-Multiplikator auf den Standardwert.
    ///
    /// The damping is multiplied by the default damping.
    pub damping_mult: f32,
}

impl Default for RoadSteeringSpringDamperManipulator {
    fn default() -> Self {
        Self {
            stiffness_add: 0.0,
            stiffness_mult: 1.0,
            damping_add: 0.0,
            damping_mult: 1.0,
        }
    }
}

impl RoadSteeringSpringDamperManipulator {
    /// Erstellt einen neuen Satz von Feder-/Dämpfer-Manipulationswerten.
    ///
    /// Creates a new spring/damper manipulation set.
    pub fn new(
        stiffness_add: f32,
        stiffness_mult: f32,
        damping_add: f32,
        damping_mult: f32,
    ) -> Self {
        Self {
            stiffness_add,
            stiffness_mult,
            damping_add,
            damping_mult,
        }
    }
}
