use serde::{Deserialize, Serialize};

use super::{Category, Difficulty};

// Re-export Decision from cortex (where Scorer trait lives)
pub use loom_cortex::bench::Decision;

/// A single benchmark sample.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchSample {
    pub id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    pub expected_decision: Decision,
    pub expected_labels: Vec<String>,
    pub primary_category: Category,
    pub difficulty: Difficulty,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
