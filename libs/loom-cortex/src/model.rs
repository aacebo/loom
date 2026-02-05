use rust_bert::pipelines::*;

use crate::{CortexModelType, CortexSentenceEmbeddingsModelType};

/// Unified model enum wrapping all rust_bert pipeline models
pub enum CortexModel {
    Conversation {
        model: conversation::ConversationModel,
        model_type: CortexModelType,
    },
    MaskedLanguage {
        model: masked_language::MaskedLanguageModel,
        model_type: CortexModelType,
    },
    Ner {
        model: ner::NERModel,
        model_type: CortexModelType,
    },
    PosTagging {
        model: pos_tagging::POSModel,
        model_type: CortexModelType,
    },
    QuestionAnswering {
        model: question_answering::QuestionAnsweringModel,
        model_type: CortexModelType,
    },
    SentenceEmbeddings {
        model: sentence_embeddings::SentenceEmbeddingsModel,
        model_type: CortexSentenceEmbeddingsModelType,
    },
    Sentiment {
        model: sentiment::SentimentModel,
        model_type: CortexModelType,
    },
    SequenceClassification {
        model: sequence_classification::SequenceClassificationModel,
        model_type: CortexModelType,
    },
    Summarization {
        model: summarization::SummarizationModel,
        model_type: CortexModelType,
    },
    TextGeneration {
        model: text_generation::TextGenerationModel,
        model_type: CortexModelType,
    },
    TokenClassification {
        model: token_classification::TokenClassificationModel,
        model_type: CortexModelType,
    },
    Translation {
        model: translation::TranslationModel,
        model_type: CortexModelType,
    },
    ZeroShotClassification {
        model: zero_shot_classification::ZeroShotClassificationModel,
        model_type: CortexModelType,
    },
}

impl CortexModel {
    pub fn category(&self) -> &'static str {
        match self {
            Self::Conversation { .. } => "conversation",
            Self::MaskedLanguage { .. } => "masked_language",
            Self::Ner { .. } => "ner",
            Self::PosTagging { .. } => "pos_tagging",
            Self::QuestionAnswering { .. } => "question_answering",
            Self::SentenceEmbeddings { .. } => "sentence_embeddings",
            Self::Sentiment { .. } => "sentiment",
            Self::SequenceClassification { .. } => "sequence_classification",
            Self::Summarization { .. } => "summarization",
            Self::TextGeneration { .. } => "text_generation",
            Self::TokenClassification { .. } => "token_classification",
            Self::Translation { .. } => "translation",
            Self::ZeroShotClassification { .. } => "zero_shot_classification",
        }
    }

    /// Returns a reference to the model type.
    /// Returns `None` for SentenceEmbeddings which uses a different model type.
    pub fn model_type(&self) -> Option<&CortexModelType> {
        match self {
            Self::Conversation { model_type, .. } => Some(model_type),
            Self::MaskedLanguage { model_type, .. } => Some(model_type),
            Self::Ner { model_type, .. } => Some(model_type),
            Self::PosTagging { model_type, .. } => Some(model_type),
            Self::QuestionAnswering { model_type, .. } => Some(model_type),
            Self::SentenceEmbeddings { .. } => None,
            Self::Sentiment { model_type, .. } => Some(model_type),
            Self::SequenceClassification { model_type, .. } => Some(model_type),
            Self::Summarization { model_type, .. } => Some(model_type),
            Self::TextGeneration { model_type, .. } => Some(model_type),
            Self::TokenClassification { model_type, .. } => Some(model_type),
            Self::Translation { model_type, .. } => Some(model_type),
            Self::ZeroShotClassification { model_type, .. } => Some(model_type),
        }
    }

    /// Returns a reference to the sentence embeddings model type.
    /// Returns `Some` only for the SentenceEmbeddings variant.
    pub fn sentence_embeddings_model_type(&self) -> Option<&CortexSentenceEmbeddingsModelType> {
        match self {
            Self::SentenceEmbeddings { model_type, .. } => Some(model_type),
            _ => None,
        }
    }

    pub fn is_conversation(&self) -> bool {
        matches!(self, Self::Conversation { .. })
    }

    pub fn is_masked_language(&self) -> bool {
        matches!(self, Self::MaskedLanguage { .. })
    }

    pub fn is_ner(&self) -> bool {
        matches!(self, Self::Ner { .. })
    }

    pub fn is_pos_tagging(&self) -> bool {
        matches!(self, Self::PosTagging { .. })
    }

    pub fn is_question_answering(&self) -> bool {
        matches!(self, Self::QuestionAnswering { .. })
    }

    pub fn is_sentence_embeddings(&self) -> bool {
        matches!(self, Self::SentenceEmbeddings { .. })
    }

    pub fn is_sentiment(&self) -> bool {
        matches!(self, Self::Sentiment { .. })
    }

    pub fn is_sequence_classification(&self) -> bool {
        matches!(self, Self::SequenceClassification { .. })
    }

    pub fn is_summarization(&self) -> bool {
        matches!(self, Self::Summarization { .. })
    }

    pub fn is_text_generation(&self) -> bool {
        matches!(self, Self::TextGeneration { .. })
    }

    pub fn is_token_classification(&self) -> bool {
        matches!(self, Self::TokenClassification { .. })
    }

    pub fn is_translation(&self) -> bool {
        matches!(self, Self::Translation { .. })
    }

    pub fn is_zero_shot_classification(&self) -> bool {
        matches!(self, Self::ZeroShotClassification { .. })
    }
}
