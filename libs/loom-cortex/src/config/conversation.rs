use rust_bert::pipelines::conversation;
use serde::{Deserialize, Serialize};

use crate::{CortexDevice, CortexModelSource, CortexModelType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexConversationConfig {
    pub model: CortexModelType,

    #[serde(default)]
    pub source: CortexModelSource,

    #[serde(default)]
    pub device: CortexDevice,

    #[serde(default = "CortexConversationConfig::default_min_length_for_response")]
    pub min_length_for_response: i64,

    #[serde(default = "CortexConversationConfig::default_max_length")]
    pub max_length: Option<i64>,

    #[serde(default)]
    pub do_sample: bool,

    #[serde(default)]
    pub num_beams: Option<i64>,
}

impl CortexConversationConfig {
    fn default_min_length_for_response() -> i64 {
        32
    }

    fn default_max_length() -> Option<i64> {
        Some(1000)
    }

    pub fn new(model: CortexModelType) -> CortexConversationConfigBuilder {
        CortexConversationConfigBuilder::new(model)
    }
}

pub struct CortexConversationConfigBuilder {
    model: CortexModelType,
    source: CortexModelSource,
    device: CortexDevice,
    min_length_for_response: i64,
    max_length: Option<i64>,
    do_sample: bool,
    num_beams: Option<i64>,
}

impl CortexConversationConfigBuilder {
    pub fn new(model: CortexModelType) -> Self {
        Self {
            model,
            source: CortexModelSource::default(),
            device: CortexDevice::default(),
            min_length_for_response: CortexConversationConfig::default_min_length_for_response(),
            max_length: CortexConversationConfig::default_max_length(),
            do_sample: false,
            num_beams: None,
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

    pub fn min_length_for_response(mut self, min_length_for_response: i64) -> Self {
        self.min_length_for_response = min_length_for_response;
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

    pub fn num_beams(mut self, num_beams: Option<i64>) -> Self {
        self.num_beams = num_beams;
        self
    }

    pub fn build(self) -> CortexConversationConfig {
        CortexConversationConfig {
            model: self.model,
            source: self.source,
            device: self.device,
            min_length_for_response: self.min_length_for_response,
            max_length: self.max_length,
            do_sample: self.do_sample,
            num_beams: self.num_beams,
        }
    }
}

impl Default for CortexConversationConfig {
    fn default() -> Self {
        Self {
            model: CortexModelType::GPT2,
            source: CortexModelSource::Default,
            device: CortexDevice::default(),
            min_length_for_response: 32,
            max_length: Some(1000),
            do_sample: false,
            num_beams: None,
        }
    }
}

impl From<CortexConversationConfig> for conversation::ConversationConfig {
    fn from(config: CortexConversationConfig) -> Self {
        let mut result = Self {
            model_type: config.model.into(),
            min_length_for_response: config.min_length_for_response,
            max_length: config.max_length,
            do_sample: config.do_sample,
            device: config.device.into(),
            ..Default::default()
        };

        if let Some(num_beams) = config.num_beams {
            result.num_beams = num_beams;
        }

        result
    }
}
