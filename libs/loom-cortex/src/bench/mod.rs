//! ML-specific benchmarking types.
//!
//! This module contains the core ML abstractions:
//! - `Scorer` trait for text scoring models
//! - `Decision` enum for accept/reject outcomes
//! - `platt` submodule for Platt calibration training
//!
//! For operational types (datasets, results, runner), see `loom_runtime::bench`.

mod decision;
pub mod platt;
mod scorer;

pub use decision::*;
pub use scorer::*;
