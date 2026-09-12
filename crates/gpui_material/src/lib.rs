//! Material 3 / Material You tokens and resolved component appearances.
//!
//! This crate is host-testable (no GPUI / Android). The Android `component_demo`
//! and the HTML catalog both consume the same `resolve` functions.

pub mod argb;
pub mod catalog;
pub mod color;
pub mod components;
pub mod elevation;
pub mod inventory;
pub mod motion;
pub mod palette;
pub mod shape;
pub mod state;
pub mod theme;
pub mod typography;

pub use argb::Argb;
pub use color::ColorScheme;
pub use inventory::{ComponentEntry, Parity, INVENTORY};
pub use state::InteractionState;
pub use theme::Theme;
