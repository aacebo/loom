# Scoring System Improvement Backlog

## Priority: Immediate

### 1. Add Missing Label Categories

Current labels don't have good "catch-all" options for low-significance content.

**ContextLabel - add:**
```rust
Greeting,    // "This text is a greeting or casual pleasantry."
```

**OutcomeLabel - add:**
```rust
Trivial,     // "This text contains no significant or memorable information."
```

Without these, the model is forced to assign scores to inappropriate labels.

---

## Priority: High

### 2. Implement Per-Label Confidence Thresholds

Some labels need stricter thresholds due to high false-positive rates:

```rust
impl Label {
    pub fn min_confidence(&self) -> f64 {
        match self {
            // High false-positive labels need stricter thresholds
            Self::Context(ContextLabel::Person) => 0.8,
            Self::Context(ContextLabel::Social) => 0.75,
            Self::Outcome(OutcomeLabel::Response) => 0.75,
            // Sentiment is generally reliable
            Self::Sentiment(_) => 0.6,
            // Default
            _ => 0.7,
        }
    }
}
```

Apply in result parsing:
```rust
if class.score >= label.min_confidence() {
    labels.push(ScoreLabel::new(label, class.sentence).with_score(class.score));
}
```

### 3. Improve Hypothesis Specificity

Some hypotheses could be more discriminating:

| Current | More Specific |
|---------|---------------|
| "This text expresses a positive sentiment." | "This text expresses approval, satisfaction, or optimism about something." |
| "This text describes a response to a prior action." | "This text describes how someone or something reacted to a previous event or decision." |
| "This text references a specific time or date." | "This text mentions a specific date, time, deadline, or scheduled event." |

---

## Priority: Medium

### 4. Change Score Aggregation Logic

Currently takes the **max score across all categories** (result.rs:20-24). This means a high `Positive` sentiment (0.9) passes even if all Context labels are low.

**Option A: Weighted average across categories**
```rust
let weights = [
    (LabelCategory::Context, 0.4),   // Most important for memory
    (LabelCategory::Outcome, 0.3),
    (LabelCategory::Emotion, 0.2),
    (LabelCategory::Sentiment, 0.1), // Least discriminating
];
```

**Option B: Require minimum scores in multiple categories**
```rust
// Must have Context OR Outcome above threshold, not just Sentiment
let context_score = self.category(LabelCategory::Context).score;
let outcome_score = self.category(LabelCategory::Outcome).score;
let passes = context_score > 0.5 || outcome_score > 0.5;
```

### 5. Add Input Length Preprocessing

Short inputs are inherently harder to classify:

```rust
impl ScoreLayer {
    fn preprocess(&self, text: &str) -> Option<&str> {
        // Skip very short inputs
        if text.split_whitespace().count() < 4 {
            return None; // Auto-cancel
        }

        // Could also: lowercase, remove punctuation, expand contractions
        Some(text)
    }
}
```

---

## Priority: Low / Future Consideration

### 6. Rethink Label Semantics for Memory Classification

Current labels mix two different concerns:
- **What the text IS** (sentiment, emotion)
- **What the text is ABOUT** (person, place, time)

For memory classification, consider labels aligned with memory significance:

| Current | Problem | Suggested Alternative |
|---------|---------|----------------------|
| `Response` | Too vague - greetings match | `ActionOutcome` - "describes the result of a deliberate action" |
| `Social` | Greetings are "social" | `Relationship` - "describes a relationship between specific people" |
| `Person` | "you" triggers it | `PersonInfo` - "contains biographical or identifying information about someone" |

### 7. Add Salience-Specific Labels

Since this is a memory system, add labels that directly measure memorability:

```rust
pub enum SalienceLabel {
    Novel,       // "This text describes something new or surprising."
    Routine,     // "This text describes everyday routine activities."
    Significant, // "This text describes a significant life event."
    Trivial,     // "This text is small talk with no lasting significance."
}
```

### 8. Use Multi-Stage Classification

Instead of running all 20+ labels at once, use a pipeline:

```
Stage 1: Salience Filter
   - Labels: [Significant, Trivial, Routine, Novel]
   - If Trivial > 0.7 -> Cancel early

Stage 2: Category Detection (only if Stage 1 passes)
   - Labels: [Emotional, Factual, Procedural, Episodic]

Stage 3: Detail Extraction
   - Run specific labels based on Stage 2 result
```

This reduces noise and improves accuracy by narrowing context.

### 9. Add Negative/Exclusion Labels

Tell the model what the text is NOT:

```rust
// Add to hypotheses
Self::Person => "This text contains information about a specific named person.",
// Add inverse check
Self::NotPerson => "This text does not mention any specific person by name.",
```

If `NotPerson` scores higher than `Person`, suppress the `Person` label.

### 10. Add Confidence Calibration

Zero-shot models can be overconfident. Add calibration:

```rust
fn calibrate_score(raw: f64, label: &Label) -> f64 {
    // Empirically determined calibration factors
    let factor = match label.category() {
        LabelCategory::Sentiment => 1.0,  // Generally well-calibrated
        LabelCategory::Emotion => 0.95,
        LabelCategory::Outcome => 0.85,   // Often overconfident
        LabelCategory::Context => 0.80,   // Most prone to false positives
    };
    raw * factor
}
```

---

## Known Issues

### "hi how are you?" misclassification

**Problem:** Simple greeting incorrectly classified as `response`, `social`, and `person`.

**Root Cause:**
- "you" triggers semantic similarity with "person"
- Greetings are inherently social interactions
- Questions pattern-match against conversational responses

**Solution:** Combination of:
1. Add `Greeting` label (immediate)
2. Improve hypothesis specificity (high priority)
3. Per-label confidence thresholds (high priority)
