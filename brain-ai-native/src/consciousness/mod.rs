//! Consciousness measurement module
//!
//! This module provides tools for measuring integrated information (Φ) across
//! different computational substrates

pub mod phi_measurement;
pub mod cross_substrate;

// Re-export main types
pub use phi_measurement::*;
pub use cross_substrate::*;
