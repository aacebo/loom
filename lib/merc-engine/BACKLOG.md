# Backlog

You've got a clean enum layout and nice `as_str` / `FromStr` ergonomics. The problems you're seeing ("hi how are you?" → response/social/person) are mostly **taxonomy + post-processing** issues, not Rust issues. Here are changes/additions that will immediately improve classification quality and make the system easier to evolve.

## Implementation Status

**Phase 1-2 Complete** (2026-01-27):
- ✅ Added `Phatic` context label for greetings/smalltalk
- ✅ Removed ambiguous `Response` outcome label
- ✅ Added `Neutral` sentiment label
- ✅ Added `Preference`, `Plan`, `Goal`, `Task` context labels
- ✅ Added `Progress`, `Conflict` outcome labels
- ✅ Removed `Social` context label (too broad/overlapping)
- ✅ Renamed `Person` → `Entity` (more general: persons, organizations, entities)
- ✅ Updated all impl blocks to use `Self` consistently
- **Total labels:** 26 (was 20)
  - Sentiment: 3 (Positive, Negative, Neutral)
  - Emotion: 7 (unchanged)
  - Outcome: 7 (Success, Failure, Reward, Punishment, Decision, Progress, Conflict)
  - Context: 9 (Fact, Time, Place, Entity, Phatic, Preference, Plan, Goal, Task)

**Remaining work:**
- Items #4 (Salience category), #5 (Entity validation/NER), #8 (Rust ergonomics - const arrays), #9 (Aggregation rules)
- Post-processing rules (if Phatic high → suppress Entity scores)
- SpeechAct category consideration

---

## ✅ 1) Add a `Phatic` (Smalltalk) context label [COMPLETED]

Right now greetings must land in `Social` / `Person` / `Response`. Give them a home.

```rust
pub enum ContextLabel {
    Fact,
    Time,
    Place,
    Person,
    Social,
    Phatic, // greeting/thanks/bye/how-are-you/pleasantries
}
```

And include it in `Label::all()` / `Label::context()`.

**Policy:** if `Phatic` is high, you typically *don't store memory* and you suppress other context labels (especially Entity) unless there's extra content.

## ✅ 2) Split "Response" into "Reply" vs "Answer" or move it out of `Outcome` [COMPLETED - Response removed]

`OutcomeLabel::Response` is ambiguous and tends to fire on any conversational utterance.

Two better options:

### Option A (recommended): make it a `SpeechAct` category instead of Outcome

New category:

```rust
pub enum Label {
    Sentiment(SentimentLabel),
    Emotion(EmotionLabel),
    Outcome(OutcomeLabel),
    Context(ContextLabel),
    SpeechAct(SpeechActLabel),
}

pub enum SpeechActLabel {
    Question,
    Answer,
    Request,
    Acknowledgement, // ok, sounds good
    Greeting,
    Farewell,
    Thanks,
    Apology,
}
```

Then remove `OutcomeLabel::Response` entirely.

### Option B: keep `Outcome`, rename `Response` → `Reply`

And add `Question` somewhere else. But honestly, “response-ness” needs **prior turn context**, so it shouldn’t be an “outcome” label at all.

## ✅ 3) Expand Context beyond "Fact/Time/Place/Entity" [COMPLETED]

The original 5 labels were too coarse, and the model would "overfit" to the closest thing.

**Implemented:**
* ✅ `Phatic` - greetings/smalltalk (from item #1)
* ✅ `Preference` - likes/dislikes/opinions
* ✅ `Plan` - commitments/future actions
* ✅ `Goal` - objectives/aspirations
* ✅ `Task` - todos/reminders
* ✅ Removed `Social` - too broad, overlapping with other labels

**Current Context labels (9 total):**
Fact, Time, Place, Entity, Phatic, Preference, Plan, Goal, Task

Add a few high-value context labels that help memory extraction:

* `Preference` (likes/dislikes)
* `Goal` / `Intent`
* `Plan` / `Commitment`
* `Task` / `Reminder`
* `Problem` / `Issue`
* `Instruction` / `HowTo`
* `PersonalDetail` (bio info; often sensitive)
* ✅ `Entity` - Renamed from `Person` to cover persons, organizations, and other named entities

A pragmatic minimal expansion:

```rust
pub enum ContextLabel {
    Fact,
    Time,
    Place,
    Entity,  // renamed from Person
    Phatic,  // added

    Preference,
    Plan,
    Goal,
    Task,
}
```

This alone reduces over-broad classification.

## 4) Add a “MemoryWorthiness / Salience” label category

This directly answers your use case (should we store memory?). Don’t infer it indirectly from the others.

```rust
pub enum Label {
    // ...
    Salience(SalienceLabel),
}

pub enum SalienceLabel {
    Store,      // should become a memory
    Ignore,     // should not
    Unsure,     // optional
}
```

Then greetings become: `Phatic + Ignore`.

## ✅ 5) Tighten semantics: redefine `Person` to require an entity signal [PARTIALLY COMPLETED - Renamed to Entity]

**Completed:**
* ✅ Renamed `Person` → `Entity` for broader coverage (persons, organizations, etc.)
* ✅ Updated hypothesis: "This text mentions a specific named person, organization, or entity."

**Remaining (Post-processing):**
Even without adding NER, you can gate `Entity` in post-processing:

* if no proper noun / @mention / email / phone / possessive relationship ("my mom") → cap `Entity` score
  This prevents "hi how are you?" from being `Entity`.

## ✅ 6) Outcome labels: keep "Decision/Success/Failure/Reward/Punishment", add "Conflict" and "Progress" [COMPLETED]

Outcomes are useful, but you’re missing two that show up a lot in real convo logs:

```rust
pub enum OutcomeLabel {
    Success,
    Failure,
    Reward,
    Punishment,
    Decision,
    // Response (remove)
    Progress,   // shipped, done, moved forward
    Conflict,   // disagreement, argument, tension
}
```

## ✅ 7) Emotion + sentiment overlap [COMPLETED - Neutral sentiment added]

You currently have both `Sentiment` (pos/neg) and `Emotion` (joy/fear/etc). That’s fine, but consider:

* Add `Neutral` sentiment (most sentences)
* Consider `Surprise` or `Disgust` if you care; otherwise leave it.

```rust
pub enum SentimentLabel {
    Negative,
    Neutral,
    Positive,
}
```

## 8) Rust ergonomics: reduce duplication and make `Label::all()` static

Your code repeats a lot of string matching. It’s okay, but you can make it more maintainable:

* Prefer `const ALL: [Label; N]` and return `&'static [Label]`.
* Consider `#[non_exhaustive]` on enums if this is a library crate you’ll evolve.
* Consider implementing `TryFrom<&str>` instead of `FromStr` if you like, but current is fine.

Example pattern:

```rust
impl Label {
    pub const ALL: [Label; 21] = [ /* ... */ ];

    pub fn all() -> &'static [Label] {
        &Self::ALL
    }
}
```

## 9) Scoring / category aggregation rule suggestion (ties back to your earlier question)

* For `Context`, use **top-k average** (k=2) so one noisy “Person” doesn’t dominate, and big categories don’t dilute.
* For `Salience(Store/Ignore)`, use **max** (a single strong “Ignore” should win for greetings).

---

### Minimal “do this now” patch list

If you want the smallest change set that fixes your greeting issue:

1. ✅ Add `ContextLabel::Phatic`
2. ✅ Remove or rename `OutcomeLabel::Response` (or move to `SpeechAct`)
3. ✅ Add `SentimentLabel::Neutral`
4. Post-process rule: if `Phatic` high ⇒ suppress `Entity` + mark `Ignore`

If you want, paste how you currently run the classifier (BART MNLI prompt/hypotheses and thresholds). I can suggest the exact hypothesis templates for `phatic`, `person`, `social`, etc., and a small deterministic Rust gate for greetings that will stop this cold.
