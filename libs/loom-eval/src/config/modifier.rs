use serde::{Deserialize, Serialize};
use serde_valid::Validate;

/// Dynamic threshold configuration based on text length.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ModifierConfig {
    /// Delta subtracted from baseline for short text
    #[serde(default = "ModifierConfig::short_text_delta")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub short_text_delta: f32,

    /// Delta added to baseline for long text
    #[serde(default = "ModifierConfig::long_text_delta")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub long_text_delta: f32,

    /// Character limit for "short" text classification (must be >= 1)
    #[serde(default = "ModifierConfig::short_text_limit")]
    #[validate(minimum = 1)]
    pub short_text_limit: usize,

    /// Character limit above which text is "long" (must be >= 1)
    #[serde(default = "ModifierConfig::long_text_limit")]
    #[validate(minimum = 1)]
    pub long_text_limit: usize,
}

impl ModifierConfig {
    fn short_text_delta() -> f32 {
        0.05
    }

    fn long_text_delta() -> f32 {
        0.05
    }

    fn short_text_limit() -> usize {
        20
    }

    fn long_text_limit() -> usize {
        200
    }
}

impl Default for ModifierConfig {
    fn default() -> Self {
        Self {
            short_text_delta: Self::short_text_delta(),
            long_text_delta: Self::long_text_delta(),
            short_text_limit: Self::short_text_limit(),
            long_text_limit: Self::long_text_limit(),
        }
    }
}
