use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    EnterTrigger(TriggerEvent),
    LeaveTrigger(TriggerEvent),
    Button(ButtonEvent),
    FloatInput(FloatInputEvent),
    Broadcast(BroadcastEvent),
    VehicleEntered(VehicleEnteredEvent),
    ReceiveMessage(ReceiveMessageEvent),
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BroadcastValue {
    Single(f32),
    String(String),
    Integer(i32),
    Serializeable(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReceiveMessageValue {
    Single(f32),
    String(String),
    Integer(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReceiveMessageKind {
    Parent,
    Child,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveMessageEvent {
    pub slot_index: i32,
    pub id: String,
    pub kind: ReceiveMessageKind,
    pub value: ReceiveMessageValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerEvent {
    pub id: String,
    pub sensor_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonEvent {
    pub id: String,
    pub value: bool,
    pub cockpit_index: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloatInputEvent {
    pub id: String,
    pub value: f32,
    pub cockpit_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastEvent {
    pub bus_id: String,
    pub id: String,
    pub value: BroadcastValue,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleEnteredEvent {
    pub id: String,
    /// Current speed in meters per second
    pub speed_mps: f32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveMessageFromChildEvent {
    pub slot_index: i32,
    pub id: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveMessageFromParentEvent {
    pub index_of_class: i32,
    pub id: String,
    pub value: i32,
}
