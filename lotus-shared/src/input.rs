use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The state kind of an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ActionStateKind {
    None,
    JustPressed,
    Pressed,
    JustReleased,
}

impl ActionStateKind {
    /// Returns `true` if the action has just been pressed.
    pub fn is_just_pressed(self) -> bool {
        matches!(self, ActionStateKind::JustPressed)
    }

    /// Returns `true` if the action is currently pressed.
    pub fn is_pressed(self) -> bool {
        matches!(
            self,
            ActionStateKind::JustPressed | ActionStateKind::Pressed
        )
    }

    /// Returns `true` if the action has just been released.
    pub fn is_just_released(self) -> bool {
        matches!(self, ActionStateKind::JustReleased)
    }

    /// Returns `true` if the action is currently released.
    pub fn is_released(self) -> bool {
        matches!(self, ActionStateKind::JustReleased | ActionStateKind::None)
    }
}

/// The state of an action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionState {
    pub kind: ActionStateKind,
    pub cockpit_index: Option<usize>,
}

macro_rules! key_code_struct {
    ($($key:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum KeyCode {
            $($key,)*
        }

        #[cfg(feature = "bevy")]
        mod _bevy {
            use super::*;

            impl TryFrom<bevy::input::keyboard::KeyCode> for KeyCode {
                type Error = ();

                fn try_from(key: bevy::input::keyboard::KeyCode) -> Result<Self, Self::Error> {
                    match key {
                        $(bevy::input::keyboard::KeyCode::$key => Ok(KeyCode::$key),)*
                        _ => Err(()),
                    }
                }
            }

            impl From<KeyCode> for bevy::input::keyboard::KeyCode {
                fn from(key: KeyCode) -> Self {
                    match key {
                        $(KeyCode::$key => bevy::input::keyboard::KeyCode::$key,)*
                    }
                }
            }
        }
    }
}

key_code_struct! {
    Space,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadDecimal,
    NumpadEnter
}
