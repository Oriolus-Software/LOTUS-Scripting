//! Gemeinsame Typen für LOTUS-Scripts und die Simulator-Engine.
//!
//! Shared types for LOTUS scripts and the simulator engine.
//!
//! Für Addon-Scripts bitte [`lotussim-script`](https://docs.rs/lotussim-script) verwenden,
//! nicht diese Crate direkt einbinden.
//!
//! For addon scripts, depend on [`lotussim-script`](https://docs.rs/lotussim-script)
//! instead of using this crate directly.

pub mod action;
pub mod animation;
pub mod content;
pub mod font;
pub mod gizmos;
pub mod graphics;
pub mod input;
pub mod math;
pub mod message;
pub mod pis;
pub mod time;
pub mod vehicle;
