mod context;
mod engine;
mod layer;
mod map;
mod options;
mod output;
pub mod score;

pub use context::*;
pub use engine::*;
pub use layer::*;
pub use map::*;
pub use options::*;
pub use output::*;

pub fn new() {}

pub trait Value: std::any::Any + std::fmt::Debug + std::fmt::Display {}

impl<T: std::any::Any + std::fmt::Debug + std::fmt::Display> Value for T {}
