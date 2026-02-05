use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsConfig, SentenceEmbeddingsModelType,
};
use serde::{Deserialize, Serialize};

use crate::CortexDevice;

/// Pre-defined sentence embeddings model types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CortexSentenceEmbeddingsModelType {
    #[default]
    AllMiniLmL12V2,
    AllMiniLmL6V2,
    AllDistilrobertaV1,
    BertBaseNliMeanTokens,
    DistiluseBaseMultilingualCased,
    ParaphraseAlbertSmallV2,
    SentenceT5Base,
}

impl CortexSentenceEmbeddingsModelType {
    pub fn is_all_mini_lm_l12_v2(&self) -> bool {
        matches!(self, Self::AllMiniLmL12V2)
    }

    pub fn is_all_mini_lm_l6_v2(&self) -> bool {
        matches!(self, Self::AllMiniLmL6V2)
    }

    pub fn is_all_distilroberta_v1(&self) -> bool {
        matches!(self, Self::AllDistilrobertaV1)
    }

    pub fn is_bert_base_nli_mean_tokens(&self) -> bool {
        matches!(self, Self::BertBaseNliMeanTokens)
    }

    pub fn is_distiluse_base_multilingual_cased(&self) -> bool {
        matches!(self, Self::DistiluseBaseMultilingualCased)
    }

    pub fn is_paraphrase_albert_small_v2(&self) -> bool {
        matches!(self, Self::ParaphraseAlbertSmallV2)
    }

    pub fn is_sentence_t5_base(&self) -> bool {
        matches!(self, Self::SentenceT5Base)
    }
}

impl From<CortexSentenceEmbeddingsModelType> for SentenceEmbeddingsModelType {
    fn from(model_type: CortexSentenceEmbeddingsModelType) -> Self {
        match model_type {
            CortexSentenceEmbeddingsModelType::AllMiniLmL12V2 => Self::AllMiniLmL12V2,
            CortexSentenceEmbeddingsModelType::AllMiniLmL6V2 => Self::AllMiniLmL6V2,
            CortexSentenceEmbeddingsModelType::AllDistilrobertaV1 => Self::AllDistilrobertaV1,
            CortexSentenceEmbeddingsModelType::BertBaseNliMeanTokens => Self::BertBaseNliMeanTokens,
            CortexSentenceEmbeddingsModelType::DistiluseBaseMultilingualCased => {
                Self::DistiluseBaseMultilingualCased
            }
            CortexSentenceEmbeddingsModelType::ParaphraseAlbertSmallV2 => {
                Self::ParaphraseAlbertSmallV2
            }
            CortexSentenceEmbeddingsModelType::SentenceT5Base => Self::SentenceT5Base,
        }
    }
}

impl From<SentenceEmbeddingsModelType> for CortexSentenceEmbeddingsModelType {
    fn from(model_type: SentenceEmbeddingsModelType) -> Self {
        match model_type {
            SentenceEmbeddingsModelType::AllMiniLmL12V2 => Self::AllMiniLmL12V2,
            SentenceEmbeddingsModelType::AllMiniLmL6V2 => Self::AllMiniLmL6V2,
            SentenceEmbeddingsModelType::AllDistilrobertaV1 => Self::AllDistilrobertaV1,
            SentenceEmbeddingsModelType::BertBaseNliMeanTokens => Self::BertBaseNliMeanTokens,
            SentenceEmbeddingsModelType::DistiluseBaseMultilingualCased => {
                Self::DistiluseBaseMultilingualCased
            }
            SentenceEmbeddingsModelType::ParaphraseAlbertSmallV2 => Self::ParaphraseAlbertSmallV2,
            SentenceEmbeddingsModelType::SentenceT5Base => Self::SentenceT5Base,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexSentenceEmbeddingsConfig {
    pub model: CortexSentenceEmbeddingsModelType,

    #[serde(default)]
    pub device: CortexDevice,
}

impl CortexSentenceEmbeddingsConfig {
    pub fn new(model: CortexSentenceEmbeddingsModelType) -> CortexSentenceEmbeddingsConfigBuilder {
        CortexSentenceEmbeddingsConfigBuilder::new(model)
    }
}

impl Default for CortexSentenceEmbeddingsConfig {
    fn default() -> Self {
        Self {
            model: CortexSentenceEmbeddingsModelType::AllMiniLmL12V2,
            device: CortexDevice::default(),
        }
    }
}

pub struct CortexSentenceEmbeddingsConfigBuilder {
    model: CortexSentenceEmbeddingsModelType,
    device: CortexDevice,
}

impl CortexSentenceEmbeddingsConfigBuilder {
    pub fn new(model: CortexSentenceEmbeddingsModelType) -> Self {
        Self {
            model,
            device: CortexDevice::default(),
        }
    }

    pub fn device(mut self, device: CortexDevice) -> Self {
        self.device = device;
        self
    }

    pub fn build(self) -> CortexSentenceEmbeddingsConfig {
        CortexSentenceEmbeddingsConfig {
            model: self.model,
            device: self.device,
        }
    }
}

impl From<CortexSentenceEmbeddingsConfig> for SentenceEmbeddingsConfig {
    fn from(config: CortexSentenceEmbeddingsConfig) -> Self {
        let model_type: SentenceEmbeddingsModelType = config.model.into();
        let mut result: Self = model_type.into();
        result.device = config.device.into();
        result
    }
}
