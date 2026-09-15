//! High-Level-API für LOTUS-Simulator-Scripts (WebAssembly).
//!
//! High-level API for LOTUS simulator scripts (WebAssembly).
//!
//! # Einstieg / Getting started
//!
//! Als Script-Autor reicht die Abhängigkeit `lotussim-script`. Im Rust-Code heißt das Crate
//! `lotus_script`.
//!
//! As a script author, depend only on `lotussim-script`. In Rust code the crate is named
//! `lotus_script`.
//!
//! ```toml
//! [dependencies]
//! lotussim-script = "0.8"
//! ```
//!
//! Scripts werden nach `wasm32-unknown-unknown` gebaut und laufen in der LOTUS-Scriptengine.
//! Der mit [`script!`] registrierte Typ muss [`Default`] implementieren.
//!
//! Scripts are compiled for `wasm32-unknown-unknown` and run inside the LOTUS script engine.
//! The type registered with [`script!`] must implement [`Default`].
//!
//! ```no_run
//! # #[cfg(target_arch = "wasm32")]
//! # {
//! use lotus_script::prelude::*;
//!
//! #[derive(Default)]
//! struct MyScript;
//!
//! impl Script for MyScript {
//!     fn init(&mut self) {}
//!
//!     fn tick(&mut self) {}
//!
//!     fn on_message(&mut self, msg: Message) {}
//! }
//!
//! script!(MyScript);
//! # }
//! ```
//!
//! # Hilfe / Help
//!
//! - Offline-Hilfe im LOTUS-Simulator / in-game offline help of the LOTUS simulator
//! - [LOTUS-Forum](https://www.lotus-simulator.de/forum/)
//! - Repository-README / repository README
//!
//! # Features
//!
//! - `time`: zusätzliche Zeitumrechnungen über die `time`-Crate / extra time conversions via the `time` crate
//! - `internal`: Schnittstellen für die Simulator-Engine, nicht für Addon-Scripts / APIs for the simulator engine, not for addon scripts

#[doc(hidden)]
pub use lotus_bindgen_macros::lotus_bindgen;

use message::Message;

pub mod action;
pub mod content;
#[doc(hidden)]
pub mod event;
pub mod font;
pub mod gizmos;
#[doc(hidden)]
pub mod global_vars;
pub mod graphics;
pub mod input;
pub mod log;
#[doc(hidden)]
pub mod macros;
pub mod math;
pub mod message;
pub mod module;
pub mod public_vars;
pub mod rand;
#[doc(hidden)]
pub mod settings;
pub mod time;
pub mod var;
pub mod vehicle;
/// PIS-Daten und Abfragefunktionen.
///
/// PIS data and query functions.
pub mod pis {
    pub use lotus_shared::pis::*;
}

/// Häufig genutzte Typen und Makros für Script-Implementierungen.
///
/// Commonly used types and macros for script implementations.
pub mod prelude {
    pub use crate::{
        action,
        graphics::{textures::Texture, Color},
        log,
        message::{message_type, send_message, Message, MessageTarget, MessageType},
        rand, script, time,
        var::{get_var, set_var, VariableType},
        vehicle, Script,
    };
}
pub use lotus_shared::animation::*;

/// Haupt-Trait für LOTUS-Script-Implementierungen.
///
/// Main trait for LOTUS script implementations.
pub trait Script {
    /// Wird einmal beim Laden des Scripts aufgerufen.
    ///
    /// Called once when the script is loaded.
    fn init(&mut self) {}

    /// Registriert Eingabeaktionen; Standard: leere Liste.
    ///
    /// Registers input actions; defaults to an empty list.
    fn actions() -> Vec<action::RegisterAction> {
        Default::default()
    }

    /// Wird pro Simulations-Tick aufgerufen.
    ///
    /// Called once per simulation tick.
    fn tick(&mut self) {}

    /// Verarbeitet eine eingehende Nachricht.
    ///
    /// Handles an incoming message.
    #[allow(unused_variables)]
    fn on_message(&mut self, msg: Message) {}
}

/// Gibt `true` zurück, wenn das scriptbehaftete Objekt ferngesteuert ist.
///
/// Returns `true` if the object the script is attached to is remote controlled.
pub fn is_rc() -> bool {
    unsafe { lotus_script_sys::env::is_rc() }
}
