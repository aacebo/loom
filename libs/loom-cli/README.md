# loom-cli

Command-line interface for the Loom scoring runtime.

## Installation

```bash
cargo build --package loom-cli
```

## Usage

```bash
loom <command> [options]
```

## Commands

### `run` - Run Evaluation

Run evaluation against a dataset and output results.

```bash
loom run <path> --config <config> [options]

Arguments:
  <path>                     Path to the dataset JSON file

Options:
  -c, --config <CONFIG>      Path to config file (YAML/JSON/TOML)
  -o, --output <DIR>         Output directory for results (default: input file's directory)
  -v, --verbose              Show detailed per-category and per-label results
      --concurrency <N>      Number of parallel inference workers (overrides config)
      --batch-size <N>       Batch size for ML inference (overrides config)
      --strict               Fail if samples have categories/labels not in config
```

Example:
```bash
loom run datasets/samples.json -c configs/score.yaml
loom run datasets/samples.json -c configs/score.yaml -v --batch-size 32
```

### `validate` - Validate Dataset

Validate a dataset for structural correctness and optionally against a config.

```bash
loom validate <path> [options]

Arguments:
  <path>                     Path to the dataset JSON file

Options:
  -c, --config <CONFIG>      Path to config file for category/label validation
      --strict               Exit with error code if validation fails
```

Checks for:
- Valid JSON structure
- Required fields present
- Valid label names
- Valid decision values (accept/reject)
- No duplicate sample IDs
- Categories/labels match config (when config provided)

Example:
```bash
loom validate datasets/samples.json
loom validate datasets/samples.json -c configs/score.yaml --strict
```

### `score` - Extract Raw Scores

Extract raw scores from a dataset for Platt calibration training.

```bash
loom score <path> --config <config> [options]

Arguments:
  <path>                     Path to the dataset JSON file

Options:
  -c, --config <CONFIG>      Path to config file (YAML/JSON/TOML)
  -o, --output <DIR>         Output directory for scores (default: input file's directory)
      --concurrency <N>      Number of parallel inference workers (overrides config)
      --batch-size <N>       Batch size for ML inference (overrides config)
      --strict               Fail if samples have categories/labels not in config
```

Outputs a `scores.json` file containing raw model scores for each sample.

Example:
```bash
loom score datasets/samples.json -c configs/score.yaml
loom score datasets/samples.json -c configs/score.yaml -o output/
```

### `train` - Train Platt Calibration

Train Platt calibration parameters from raw scores.

```bash
loom train <path> --output <output> [options]

Arguments:
  <path>                     Path to raw scores JSON (from score command)

Options:
  -o, --output <OUTPUT>      Output path for trained parameters JSON (required)
      --code                 Also output Rust code for label.rs
```

Example:
```bash
loom train output/scores.json -o output/params.json
loom train output/scores.json -o output/params.json --code
```

## Configuration

The CLI supports configuration via YAML, JSON, or TOML files. Settings can be overridden using environment variables with the `LOOM_` prefix.

Environment variable mapping (after prefix removal):
- Single `_` becomes `.` (hierarchy separator)
- Double `__` becomes literal `_` in key name

Examples:
- `LOOM_CONCURRENCY=16` -> `concurrency: 16`
- `LOOM_BATCH__SIZE=32` -> `batch_size: 32`
- `LOOM_LAYERS_SCORE_THRESHOLD=0.8` -> `layers.score.threshold: 0.8`

## Development

Run with cargo:
```bash
cargo run --package loom-cli -- run datasets/samples.json -c configs/score.yaml
cargo run --package loom-cli -- validate datasets/samples.json
```

## Navigation

[← Back to Libraries](../README.md)
