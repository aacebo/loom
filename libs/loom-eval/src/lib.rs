pub mod config;
mod dataset;
mod difficulty;
mod layer;
mod output;
pub mod result;
mod sample;
mod validation;

// Config types
pub use config::{CategoryConfig, EvalConfig, LabelConfig, ModifierConfig};

// Core types
pub use dataset::SampleDataset;
pub use difficulty::Difficulty;
pub use layer::EvalLayer;
pub use output::{CategoryOutput, EvalOutput, LabelOutput};
pub use sample::{Decision, Sample};
pub use validation::ValidationError;

// Result types
pub use result::{
    CategoryMetrics, CategoryResult, EvalMetrics, EvalResult, LabelMetrics, LabelResult,
    SampleResult,
};
