# loom-eval

Evaluation pipeline layer for Loom providing scoring, result aggregation, Platt calibration, and metrics computation.

## Overview

`loom-eval` is a pipeline layer crate that implements the `Layer` trait from `loom-pipe`. It uses zero-shot classification (via `loom-cortex`) to score text across configured categories and labels, then aggregates results with Platt calibration.

## Key Types

| Type | Description |
|------|-------------|
| **EvalLayer** | Pipeline layer implementing `Layer<Input = RunContext>`; scores text via zero-shot classification |
| **EvalConfig** | Configuration for categories, labels, weights, thresholds, and model settings |
| **CategoryConfig** | Per-category configuration with labels and top-k setting |
| **LabelConfig** | Per-label hypothesis, weight, threshold, and Platt calibration parameters |
| **ModifierConfig** | Threshold modifiers based on text length |
| **EvalOutput** | Scoring output with overall score and per-category/label breakdowns |
| **CategoryOutput** | Per-category score computed from top-k labels |
| **LabelOutput** | Per-label calibrated score, raw score, and sentence index |
| **EvalResult** | Aggregated evaluation results with counts, per-category, and per-label breakdowns |
| **SampleResult** | Result for a single evaluated sample |
| **EvalMetrics** | Computed metrics (accuracy, precision, recall, F1) from an EvalResult |

## Key Methods

### EvalLayer

- `EvalLayer::from_config(config: &Config) -> Result<Self>` -- Build an EvalLayer from a `loom_config::Config` by reading the `layers.eval` section.
- `layer.score(text: &str) -> Result<EvalOutput>` -- Score a single text string and return the eval output.
- `layer.valid_categories() -> Vec<String>` -- Get all valid category names from the config.
- `layer.valid_labels() -> Vec<String>` -- Get all valid label names from the config.

### EvalOutput

- `output.to_result(sample: &Sample, threshold: f32) -> EvalResult` -- Convert an output into an EvalResult for a single sample.
- `output.decide(threshold: f32) -> Decision` -- Decide Accept/Reject based on the given threshold.
- `output.detected_labels() -> Vec<String>` -- Get labels whose score is above zero.
- `output.raw_scores() -> Vec<(String, f32)>` -- Get raw (label, score) pairs.

### EvalResult

- `result.merge(other: EvalResult) -> EvalResult` -- Merge another result into this one, combining all counts and sample results.
- `result.metrics() -> EvalMetrics` -- Compute accuracy, precision, recall, and F1 from the accumulated counts.
- `result.accumulate(sample, sample_result)` -- Accumulate a single sample's results into running totals.

## Layer Implementation

`EvalLayer` implements `Layer` with `type Input = RunContext`. When `process()` is called, it:

1. Reads the input text from `ctx.input()`
2. Runs zero-shot classification via the cortex model
3. Emits an `eval.scored` signal via `ctx.emit()` with the score
4. Returns the `EvalOutput` as a `Value`

```rust
impl Layer for EvalLayer {
    type Input = RunContext;

    fn process(&self, ctx: &RunContext) -> Result<Value> {
        let text = ctx.input().as_str().unwrap_or_default();
        let eval_output = self.score(text)?;
        ctx.emit("eval.scored", &attrs);
        Ok(eval_output.into())
    }
}
```

## Usage

```rust
use loom_eval::EvalLayer;
use loom_runtime::Runtime;
use loom_config::Config;

// Build the eval layer from config
let config = Config::new()
    .file("config.yaml")
    .env("LOOM_")
    .build()?;

let eval_layer = EvalLayer::from_config(&config)?;

// Add it to a runtime
let runtime = Runtime::new()
    .layer(eval_layer)
    .build();

// Execute
let result = runtime.execute("some text to evaluate")?;

// Convert output to eval result
let output: EvalOutput = result.try_into()?;
let eval_result = output.to_result(&sample, 0.75);
let metrics = eval_result.metrics();
```

## Navigation

[<- Back to Libraries](../README.md)
