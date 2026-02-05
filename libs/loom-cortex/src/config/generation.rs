use rust_bert::pipelines::{summarization, text_generation};
use serde::{Deserialize, Serialize};

use crate::{CortexDevice, CortexModelSource, CortexModelType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexGenerationConfig {
    pub model: CortexModelType,

    #[serde(default)]
    pub source: CortexModelSource,

    #[serde(default)]
    pub device: CortexDevice,

    #[serde(default = "CortexGenerationConfig::default_min_length")]
    pub min_length: i64,

    #[serde(default = "CortexGenerationConfig::default_max_length")]
    pub max_length: Option<i64>,

    #[serde(default)]
    pub do_sample: bool,

    #[serde(default = "CortexGenerationConfig::default_early_stopping")]
    pub early_stopping: bool,

    #[serde(default = "CortexGenerationConfig::default_num_beams")]
    pub num_beams: i64,

    #[serde(default = "CortexGenerationConfig::default_temperature")]
    pub temperature: f64,

    #[serde(default = "CortexGenerationConfig::default_top_k")]
    pub top_k: i64,

    #[serde(default = "CortexGenerationConfig::default_top_p")]
    pub top_p: f64,

    #[serde(default = "CortexGenerationConfig::default_repetition_penalty")]
    pub repetition_penalty: f64,

    #[serde(default = "CortexGenerationConfig::default_length_penalty")]
    pub length_penalty: f64,

    #[serde(default = "CortexGenerationConfig::default_no_repeat_ngram_size")]
    pub no_repeat_ngram_size: i64,

    #[serde(default = "CortexGenerationConfig::default_num_return_sequences")]
    pub num_return_sequences: i64,
}

impl CortexGenerationConfig {
    fn default_min_length() -> i64 {
        0
    }

    fn default_max_length() -> Option<i64> {
        Some(56)
    }

    fn default_early_stopping() -> bool {
        true
    }

    fn default_num_beams() -> i64 {
        5
    }

    fn default_temperature() -> f64 {
        1.0
    }

    fn default_top_k() -> i64 {
        50
    }

    fn default_top_p() -> f64 {
        1.0
    }

    fn default_repetition_penalty() -> f64 {
        1.0
    }

    fn default_length_penalty() -> f64 {
        1.0
    }

    fn default_no_repeat_ngram_size() -> i64 {
        3
    }

    fn default_num_return_sequences() -> i64 {
        1
    }

    pub fn new(model: CortexModelType) -> CortexGenerationConfigBuilder {
        CortexGenerationConfigBuilder::new(model)
    }

    pub fn into_summarization_config(self) -> summarization::SummarizationConfig {
        summarization::SummarizationConfig {
            model_type: self.model.into(),
            device: self.device.into(),
            min_length: self.min_length,
            max_length: self.max_length,
            do_sample: self.do_sample,
            early_stopping: self.early_stopping,
            num_beams: self.num_beams,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
            repetition_penalty: self.repetition_penalty,
            length_penalty: self.length_penalty,
            no_repeat_ngram_size: self.no_repeat_ngram_size,
            num_return_sequences: self.num_return_sequences,
            ..Default::default()
        }
    }

    pub fn into_text_generation_config(self) -> text_generation::TextGenerationConfig {
        text_generation::TextGenerationConfig {
            model_type: self.model.into(),
            device: self.device.into(),
            min_length: self.min_length,
            max_length: self.max_length,
            do_sample: self.do_sample,
            early_stopping: self.early_stopping,
            num_beams: self.num_beams,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
            repetition_penalty: self.repetition_penalty,
            length_penalty: self.length_penalty,
            no_repeat_ngram_size: self.no_repeat_ngram_size,
            num_return_sequences: self.num_return_sequences,
            ..Default::default()
        }
    }
}

impl Default for CortexGenerationConfig {
    fn default() -> Self {
        Self {
            model: CortexModelType::Bart,
            source: CortexModelSource::Default,
            device: CortexDevice::default(),
            min_length: 0,
            max_length: Some(56),
            do_sample: false,
            early_stopping: true,
            num_beams: 5,
            temperature: 1.0,
            top_k: 50,
            top_p: 1.0,
            repetition_penalty: 1.0,
            length_penalty: 1.0,
            no_repeat_ngram_size: 3,
            num_return_sequences: 1,
        }
    }
}

pub struct CortexGenerationConfigBuilder {
    model: CortexModelType,
    source: CortexModelSource,
    device: CortexDevice,
    min_length: i64,
    max_length: Option<i64>,
    do_sample: bool,
    early_stopping: bool,
    num_beams: i64,
    temperature: f64,
    top_k: i64,
    top_p: f64,
    repetition_penalty: f64,
    length_penalty: f64,
    no_repeat_ngram_size: i64,
    num_return_sequences: i64,
}

impl CortexGenerationConfigBuilder {
    pub fn new(model: CortexModelType) -> Self {
        Self {
            model,
            source: CortexModelSource::default(),
            device: CortexDevice::default(),
            min_length: CortexGenerationConfig::default_min_length(),
            max_length: CortexGenerationConfig::default_max_length(),
            do_sample: false,
            early_stopping: CortexGenerationConfig::default_early_stopping(),
            num_beams: CortexGenerationConfig::default_num_beams(),
            temperature: CortexGenerationConfig::default_temperature(),
            top_k: CortexGenerationConfig::default_top_k(),
            top_p: CortexGenerationConfig::default_top_p(),
            repetition_penalty: CortexGenerationConfig::default_repetition_penalty(),
            length_penalty: CortexGenerationConfig::default_length_penalty(),
            no_repeat_ngram_size: CortexGenerationConfig::default_no_repeat_ngram_size(),
            num_return_sequences: CortexGenerationConfig::default_num_return_sequences(),
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

    pub fn min_length(mut self, min_length: i64) -> Self {
        self.min_length = min_length;
        self
    }

    pub fn max_length(mut self, max_length: Option<i64>) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn do_sample(mut self, do_sample: bool) -> Self {
        self.do_sample = do_sample;
        self
    }

    pub fn early_stopping(mut self, early_stopping: bool) -> Self {
        self.early_stopping = early_stopping;
        self
    }

    pub fn num_beams(mut self, num_beams: i64) -> Self {
        self.num_beams = num_beams;
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn top_k(mut self, top_k: i64) -> Self {
        self.top_k = top_k;
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn repetition_penalty(mut self, repetition_penalty: f64) -> Self {
        self.repetition_penalty = repetition_penalty;
        self
    }

    pub fn length_penalty(mut self, length_penalty: f64) -> Self {
        self.length_penalty = length_penalty;
        self
    }

    pub fn no_repeat_ngram_size(mut self, no_repeat_ngram_size: i64) -> Self {
        self.no_repeat_ngram_size = no_repeat_ngram_size;
        self
    }

    pub fn num_return_sequences(mut self, num_return_sequences: i64) -> Self {
        self.num_return_sequences = num_return_sequences;
        self
    }

    pub fn build(self) -> CortexGenerationConfig {
        CortexGenerationConfig {
            model: self.model,
            source: self.source,
            device: self.device,
            min_length: self.min_length,
            max_length: self.max_length,
            do_sample: self.do_sample,
            early_stopping: self.early_stopping,
            num_beams: self.num_beams,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
            repetition_penalty: self.repetition_penalty,
            length_penalty: self.length_penalty,
            no_repeat_ngram_size: self.no_repeat_ngram_size,
            num_return_sequences: self.num_return_sequences,
        }
    }
}
