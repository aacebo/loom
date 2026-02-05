use rust_bert::pipelines::question_answering;
use serde::{Deserialize, Serialize};

use crate::{CortexDevice, CortexModelSource, CortexModelType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexQuestionAnsweringConfig {
    pub model: CortexModelType,

    #[serde(default)]
    pub source: CortexModelSource,

    #[serde(default)]
    pub device: CortexDevice,

    #[serde(default)]
    pub lower_case: bool,

    #[serde(default)]
    pub strip_accents: Option<bool>,

    #[serde(default)]
    pub add_prefix_space: Option<bool>,

    #[serde(default = "CortexQuestionAnsweringConfig::default_max_answer_length")]
    pub max_answer_length: usize,

    #[serde(default = "CortexQuestionAnsweringConfig::default_max_query_length")]
    pub max_query_length: usize,

    #[serde(default = "CortexQuestionAnsweringConfig::default_max_seq_length")]
    pub max_seq_length: usize,
}

impl CortexQuestionAnsweringConfig {
    fn default_max_answer_length() -> usize {
        15
    }

    fn default_max_query_length() -> usize {
        64
    }

    fn default_max_seq_length() -> usize {
        384
    }

    pub fn new(model: CortexModelType) -> CortexQuestionAnsweringConfigBuilder {
        CortexQuestionAnsweringConfigBuilder::new(model)
    }
}

pub struct CortexQuestionAnsweringConfigBuilder {
    model: CortexModelType,
    source: CortexModelSource,
    device: CortexDevice,
    lower_case: bool,
    strip_accents: Option<bool>,
    add_prefix_space: Option<bool>,
    max_answer_length: usize,
    max_query_length: usize,
    max_seq_length: usize,
}

impl CortexQuestionAnsweringConfigBuilder {
    pub fn new(model: CortexModelType) -> Self {
        Self {
            model,
            source: CortexModelSource::default(),
            device: CortexDevice::default(),
            lower_case: false,
            strip_accents: None,
            add_prefix_space: None,
            max_answer_length: CortexQuestionAnsweringConfig::default_max_answer_length(),
            max_query_length: CortexQuestionAnsweringConfig::default_max_query_length(),
            max_seq_length: CortexQuestionAnsweringConfig::default_max_seq_length(),
        }
    }

    pub fn source(mut self, source: CortexModelSource) -> Self {
        self.source = source;
        self
    }

    pub fn device(mut self, device: CortexDevice) -> Self {
        self.device = device;
        self
    }

    pub fn lower_case(mut self, lower_case: bool) -> Self {
        self.lower_case = lower_case;
        self
    }

    pub fn strip_accents(mut self, strip_accents: Option<bool>) -> Self {
        self.strip_accents = strip_accents;
        self
    }

    pub fn add_prefix_space(mut self, add_prefix_space: Option<bool>) -> Self {
        self.add_prefix_space = add_prefix_space;
        self
    }

    pub fn max_answer_length(mut self, max_answer_length: usize) -> Self {
        self.max_answer_length = max_answer_length;
        self
    }

    pub fn max_query_length(mut self, max_query_length: usize) -> Self {
        self.max_query_length = max_query_length;
        self
    }

    pub fn max_seq_length(mut self, max_seq_length: usize) -> Self {
        self.max_seq_length = max_seq_length;
        self
    }

    pub fn build(self) -> CortexQuestionAnsweringConfig {
        CortexQuestionAnsweringConfig {
            model: self.model,
            source: self.source,
            device: self.device,
            lower_case: self.lower_case,
            strip_accents: self.strip_accents,
            add_prefix_space: self.add_prefix_space,
            max_answer_length: self.max_answer_length,
            max_query_length: self.max_query_length,
            max_seq_length: self.max_seq_length,
        }
    }
}

impl Default for CortexQuestionAnsweringConfig {
    fn default() -> Self {
        Self {
            model: CortexModelType::DistilBert,
            source: CortexModelSource::Default,
            device: CortexDevice::default(),
            lower_case: false,
            strip_accents: None,
            add_prefix_space: None,
            max_answer_length: 15,
            max_query_length: 64,
            max_seq_length: 384,
        }
    }
}

impl From<CortexQuestionAnsweringConfig> for question_answering::QuestionAnsweringConfig {
    fn from(config: CortexQuestionAnsweringConfig) -> Self {
        Self {
            model_type: config.model.into(),
            device: config.device.into(),
            lower_case: config.lower_case,
            strip_accents: config.strip_accents,
            add_prefix_space: config.add_prefix_space,
            max_answer_length: config.max_answer_length,
            max_query_length: config.max_query_length,
            max_seq_length: config.max_seq_length,
            ..Default::default()
        }
    }
}
