use std::str::FromStr;

use merc_error::Error;

use crate::score::LabelCategory;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Label {
    Sentiment(SentimentLabel),
    Emotion(EmotionLabel),
    Outcome(OutcomeLabel),
    Context(ContextLabel),
}

impl Label {
    pub fn all() -> [Self; 26] {
        [
            Self::Sentiment(SentimentLabel::Positive),
            Self::Sentiment(SentimentLabel::Negative),
            Self::Sentiment(SentimentLabel::Neutral),
            Self::Emotion(EmotionLabel::Joy),
            Self::Emotion(EmotionLabel::Fear),
            Self::Emotion(EmotionLabel::Shame),
            Self::Emotion(EmotionLabel::Pride),
            Self::Emotion(EmotionLabel::Stress),
            Self::Emotion(EmotionLabel::Anger),
            Self::Emotion(EmotionLabel::Sad),
            Self::Outcome(OutcomeLabel::Success),
            Self::Outcome(OutcomeLabel::Failure),
            Self::Outcome(OutcomeLabel::Reward),
            Self::Outcome(OutcomeLabel::Punishment),
            Self::Outcome(OutcomeLabel::Decision),
            Self::Outcome(OutcomeLabel::Progress),
            Self::Outcome(OutcomeLabel::Conflict),
            Self::Context(ContextLabel::Fact),
            Self::Context(ContextLabel::Time),
            Self::Context(ContextLabel::Place),
            Self::Context(ContextLabel::Entity),
            Self::Context(ContextLabel::Phatic),
            Self::Context(ContextLabel::Preference),
            Self::Context(ContextLabel::Plan),
            Self::Context(ContextLabel::Goal),
            Self::Context(ContextLabel::Task),
        ]
    }

    pub fn sentiment() -> [Self; 3] {
        [
            Self::Sentiment(SentimentLabel::Positive),
            Self::Sentiment(SentimentLabel::Negative),
            Self::Sentiment(SentimentLabel::Neutral),
        ]
    }

    pub fn emotion() -> [Self; 7] {
        [
            Self::Emotion(EmotionLabel::Joy),
            Self::Emotion(EmotionLabel::Fear),
            Self::Emotion(EmotionLabel::Shame),
            Self::Emotion(EmotionLabel::Pride),
            Self::Emotion(EmotionLabel::Stress),
            Self::Emotion(EmotionLabel::Anger),
            Self::Emotion(EmotionLabel::Sad),
        ]
    }

    pub fn outcome() -> [Self; 7] {
        [
            Self::Outcome(OutcomeLabel::Success),
            Self::Outcome(OutcomeLabel::Failure),
            Self::Outcome(OutcomeLabel::Reward),
            Self::Outcome(OutcomeLabel::Punishment),
            Self::Outcome(OutcomeLabel::Decision),
            Self::Outcome(OutcomeLabel::Progress),
            Self::Outcome(OutcomeLabel::Conflict),
        ]
    }

    pub fn context() -> [Self; 9] {
        [
            Self::Context(ContextLabel::Fact),
            Self::Context(ContextLabel::Time),
            Self::Context(ContextLabel::Place),
            Self::Context(ContextLabel::Entity),
            Self::Context(ContextLabel::Phatic),
            Self::Context(ContextLabel::Preference),
            Self::Context(ContextLabel::Plan),
            Self::Context(ContextLabel::Goal),
            Self::Context(ContextLabel::Task),
        ]
    }

    pub fn category(&self) -> LabelCategory {
        match self {
            Self::Sentiment(_) => LabelCategory::Sentiment,
            Self::Emotion(_) => LabelCategory::Emotion,
            Self::Outcome(_) => LabelCategory::Outcome,
            Self::Context(_) => LabelCategory::Context,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sentiment(v) => v.as_str(),
            Self::Emotion(v) => v.as_str(),
            Self::Outcome(v) => v.as_str(),
            Self::Context(v) => v.as_str(),
        }
    }

    pub fn hypothesis(&self) -> &'static str {
        match self {
            Self::Sentiment(v) => v.hypothesis(),
            Self::Emotion(v) => v.hypothesis(),
            Self::Outcome(v) => v.hypothesis(),
            Self::Context(v) => v.hypothesis(),
        }
    }

    pub fn threshold(&self) -> f32 {
        match self {
            Self::Sentiment(v) => v.threshold(),
            Self::Emotion(v) => v.threshold(),
            Self::Outcome(v) => v.threshold(),
            Self::Context(v) => v.threshold(),
        }
    }

    pub fn weight(&self) -> f32 {
        match self {
            Self::Sentiment(v) => v.weight(),
            Self::Emotion(v) => v.weight(),
            Self::Outcome(v) => v.weight(),
            Self::Context(v) => v.weight(),
        }
    }
}

impl FromStr for Label {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = SentimentLabel::from_str(s) {
            return Ok(Self::Sentiment(v));
        }

        if let Ok(v) = EmotionLabel::from_str(s) {
            return Ok(Self::Emotion(v));
        }

        if let Ok(v) = OutcomeLabel::from_str(s) {
            return Ok(Self::Outcome(v));
        }

        if let Ok(v) = ContextLabel::from_str(s) {
            return Ok(Self::Context(v));
        }

        Err(Error::builder()
            .message(&format!("'{}' is not a valid label", s))
            .build())
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sentiment(v) => write!(f, "{}", v),
            Self::Emotion(v) => write!(f, "{}", v),
            Self::Outcome(v) => write!(f, "{}", v),
            Self::Context(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SentimentLabel {
    Negative,
    Neutral,
    Positive,
}

impl SentimentLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
            Self::Neutral => "neutral",
        }
    }

    pub fn hypothesis(&self) -> &'static str {
        match self {
            Self::Positive => "This text expresses a positive sentiment.",
            Self::Negative => "This text expresses a negative sentiment.",
            Self::Neutral => "This text expresses a neutral sentiment.",
        }
    }

    pub fn weight(&self) -> f32 {
        match self {
            Self::Negative => 0.35,
            Self::Positive => 0.30,
            Self::Neutral => 0.10,
        }
    }

    pub fn threshold(&self) -> f32 {
        0.70
    }
}

impl From<SentimentLabel> for Label {
    fn from(value: SentimentLabel) -> Self {
        Self::Sentiment(value)
    }
}

impl FromStr for SentimentLabel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "positive" => Ok(Self::Positive),
            "negative" => Ok(Self::Negative),
            "neutral" => Ok(Self::Neutral),
            v => Err(Error::builder()
                .message(&format!("'{}' is not a valid sentiment label", v))
                .build()),
        }
    }
}

impl std::fmt::Display for SentimentLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive => write!(f, "positive"),
            Self::Negative => write!(f, "negative"),
            Self::Neutral => write!(f, "neutral"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EmotionLabel {
    Joy,
    Fear,
    Shame,
    Pride,
    Stress,
    Anger,
    Sad,
}

impl EmotionLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Joy => "joy",
            Self::Fear => "fear",
            Self::Shame => "shame",
            Self::Pride => "pride",
            Self::Stress => "stress",
            Self::Anger => "anger",
            Self::Sad => "sad",
        }
    }

    pub fn hypothesis(&self) -> &'static str {
        match self {
            Self::Joy => "This text expresses joy or happiness.",
            Self::Fear => "This text expresses fear or anxiety.",
            Self::Shame => "This text expresses shame or embarrassment.",
            Self::Pride => "This text expresses pride or accomplishment.",
            Self::Stress => "This text expresses stress or pressure.",
            Self::Anger => "This text expresses anger or frustration.",
            Self::Sad => "This text expresses sadness or grief.",
        }
    }

    pub fn weight(&self) -> f32 {
        match self {
            // Emotion/sentiment (low-medium; mostly metadata)
            Self::Stress => 0.45,
            Self::Fear => 0.40,
            Self::Anger => 0.40,
            Self::Sad => 0.40,
            Self::Shame => 0.35,
            Self::Pride => 0.30,
            Self::Joy => 0.30,
        }
    }

    pub fn threshold(&self) -> f32 {
        0.70
    }
}

impl From<EmotionLabel> for Label {
    fn from(value: EmotionLabel) -> Self {
        Self::Emotion(value)
    }
}

impl FromStr for EmotionLabel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "joy" => Ok(Self::Joy),
            "fear" => Ok(Self::Fear),
            "shame" => Ok(Self::Shame),
            "pride" => Ok(Self::Pride),
            "stress" => Ok(Self::Stress),
            "anger" => Ok(Self::Anger),
            "sad" => Ok(Self::Sad),
            v => Err(Error::builder()
                .message(&format!("'{}' is not a valid emotion label", v))
                .build()),
        }
    }
}

impl std::fmt::Display for EmotionLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Joy => write!(f, "joy"),
            Self::Fear => write!(f, "fear"),
            Self::Shame => write!(f, "shame"),
            Self::Pride => write!(f, "pride"),
            Self::Stress => write!(f, "stress"),
            Self::Anger => write!(f, "anger"),
            Self::Sad => write!(f, "sad"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OutcomeLabel {
    Success,
    Failure,
    Reward,
    Punishment,
    Decision,
    Progress,
    Conflict,
}

impl OutcomeLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::Reward => "reward",
            Self::Punishment => "punishment",
            Self::Decision => "decision",
            Self::Progress => "progress",
            Self::Conflict => "conflict",
        }
    }

    pub fn hypothesis(&self) -> &'static str {
        match self {
            Self::Success => "This text describes achieving a goal or success.",
            Self::Failure => "This text describes a failure or setback.",
            Self::Reward => "This text describes receiving a reward or benefit.",
            Self::Punishment => "This text describes a punishment or consequence.",
            Self::Decision => "This text describes making a decision or choice.",
            Self::Progress => {
                "This text describes progress, completion, or forward movement on something."
            }
            Self::Conflict => "This text describes disagreement, conflict, argument, or tension.",
        }
    }

    pub fn weight(&self) -> f32 {
        match self {
            // Outcome (medium-high)
            Self::Decision => 0.80,
            Self::Progress => 0.65,
            Self::Conflict => 0.65,
            Self::Success => 0.55,
            Self::Failure => 0.55,
            Self::Reward => 0.45,
            Self::Punishment => 0.45,
        }
    }

    pub fn threshold(&self) -> f32 {
        0.70
    }
}

impl From<OutcomeLabel> for Label {
    fn from(value: OutcomeLabel) -> Self {
        Self::Outcome(value)
    }
}

impl FromStr for OutcomeLabel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "success" => Ok(Self::Success),
            "failure" => Ok(Self::Failure),
            "reward" => Ok(Self::Reward),
            "punishment" => Ok(Self::Punishment),
            "decision" => Ok(Self::Decision),
            "progress" => Ok(Self::Progress),
            "conflict" => Ok(Self::Conflict),
            v => Err(Error::builder()
                .message(&format!("'{}' is not a valid outcome label", v))
                .build()),
        }
    }
}

impl std::fmt::Display for OutcomeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Failure => write!(f, "failure"),
            Self::Reward => write!(f, "reward"),
            Self::Punishment => write!(f, "punishment"),
            Self::Decision => write!(f, "decision"),
            Self::Progress => write!(f, "progress"),
            Self::Conflict => write!(f, "conflict"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ContextLabel {
    Fact,
    Time,
    Place,
    Entity,
    Phatic,
    Preference,
    Plan,
    Goal,
    Task,
}

impl ContextLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fact => "fact",
            Self::Time => "time",
            Self::Place => "place",
            Self::Entity => "entity",
            Self::Phatic => "phatic",
            Self::Preference => "preference",
            Self::Plan => "plan",
            Self::Goal => "goal",
            Self::Task => "task",
        }
    }

    pub fn hypothesis(&self) -> &'static str {
        match self {
            Self::Fact => "This text states a factual piece of information.",
            Self::Time => "This text references a specific time or date.",
            Self::Place => "This text references a specific location or place.",
            Self::Entity => "This text mentions a specific named person, organization, or entity.",
            Self::Phatic => "This text is a greeting, thanks, farewell, or polite small talk.",
            Self::Preference => "This text expresses a preference, like, dislike, or opinion.",
            Self::Plan => "This text describes a plan, commitment, or intention for future action.",
            Self::Goal => "This text describes a goal, objective, or aspiration.",
            Self::Task => "This text describes a task, todo item, or reminder.",
        }
    }

    pub fn weight(&self) -> f32 {
        match self {
            // Memory-bearing context (high impact)
            Self::Task => 1.00,
            Self::Plan => 0.90,
            Self::Goal => 0.90,
            Self::Preference => 0.85,
            Self::Fact => 0.80,

            // Useful metadata/context (medium)
            Self::Entity => 0.65,
            Self::Time => 0.55,
            Self::Place => 0.55,

            // Phatic should be strong as a detector, but not “memory importance”
            Self::Phatic => 0.40,
        }
    }

    pub fn threshold(&self) -> f32 {
        match self {
            // Special / noisy labels: require higher confidence
            Self::Phatic => 0.80,
            Self::Entity => 0.75,
            // Memory-bearing: allow a bit lower to catch more
            Self::Task => 0.65,
            Self::Plan => 0.65,
            Self::Goal => 0.65,
            Self::Preference => 0.65,
            // Default
            _ => 0.70,
        }
    }
}

impl From<ContextLabel> for Label {
    fn from(value: ContextLabel) -> Self {
        Self::Context(value)
    }
}

impl FromStr for ContextLabel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fact" => Ok(Self::Fact),
            "time" => Ok(Self::Time),
            "place" => Ok(Self::Place),
            "entity" => Ok(Self::Entity),
            "phatic" => Ok(Self::Phatic),
            "preference" => Ok(Self::Preference),
            "plan" => Ok(Self::Plan),
            "goal" => Ok(Self::Goal),
            "task" => Ok(Self::Task),
            v => Err(Error::builder()
                .message(&format!("'{}' is not a valid context label", v))
                .build()),
        }
    }
}

impl std::fmt::Display for ContextLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fact => write!(f, "fact"),
            Self::Time => write!(f, "time"),
            Self::Place => write!(f, "place"),
            Self::Entity => write!(f, "entity"),
            Self::Phatic => write!(f, "phatic"),
            Self::Preference => write!(f, "preference"),
            Self::Plan => write!(f, "plan"),
            Self::Goal => write!(f, "goal"),
            Self::Task => write!(f, "task"),
        }
    }
}
