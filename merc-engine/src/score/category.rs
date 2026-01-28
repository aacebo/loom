use crate::score::{ContextLabel, EmotionLabel, Label, OutcomeLabel, SentimentLabel};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LabelCategory {
    Sentiment,
    Emotion,
    Outcome,
    Context,
}

impl LabelCategory {
    pub fn labels(self) -> &'static [Label] {
        match self {
            Self::Sentiment => &[
                Label::Sentiment(SentimentLabel::Positive),
                Label::Sentiment(SentimentLabel::Negative),
                Label::Sentiment(SentimentLabel::Neutral),
            ],
            Self::Emotion => &[
                Label::Emotion(EmotionLabel::Joy),
                Label::Emotion(EmotionLabel::Fear),
                Label::Emotion(EmotionLabel::Shame),
                Label::Emotion(EmotionLabel::Pride),
                Label::Emotion(EmotionLabel::Stress),
                Label::Emotion(EmotionLabel::Anger),
                Label::Emotion(EmotionLabel::Sad),
            ],
            Self::Outcome => &[
                Label::Outcome(OutcomeLabel::Success),
                Label::Outcome(OutcomeLabel::Failure),
                Label::Outcome(OutcomeLabel::Reward),
                Label::Outcome(OutcomeLabel::Punishment),
                Label::Outcome(OutcomeLabel::Decision),
                Label::Outcome(OutcomeLabel::Progress),
                Label::Outcome(OutcomeLabel::Conflict),
            ],
            Self::Context => &[
                Label::Context(ContextLabel::Fact),
                Label::Context(ContextLabel::Time),
                Label::Context(ContextLabel::Place),
                Label::Context(ContextLabel::Entity),
                Label::Context(ContextLabel::Phatic),
                Label::Context(ContextLabel::Preference),
                Label::Context(ContextLabel::Plan),
                Label::Context(ContextLabel::Goal),
                Label::Context(ContextLabel::Task),
            ],
        }
    }
}

impl std::fmt::Display for LabelCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sentiment => write!(f, "sentiment"),
            Self::Emotion => write!(f, "emotion"),
            Self::Outcome => write!(f, "outcome"),
            Self::Context => write!(f, "context"),
        }
    }
}
