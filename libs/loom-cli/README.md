# loom-cli

Command-line interface for the Loom runtime.

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
```

Example:
```bash
loom run datasets/samples.json -c configs/eval.yaml
loom run datasets/samples.json -c configs/eval.yaml -v
loom run datasets/samples.json -c configs/eval.yaml -o output/ -v
```

## Configuration

The CLI supports configuration via YAML, JSON, or TOML files. Settings can be overridden using environment variables with the `LOOM_` prefix.

Environment variable mapping (after prefix removal):
- Single `_` becomes `.` (hierarchy separator)
- Double `__` becomes literal `_` in key name

Examples:
- `LOOM_LAYERS_EVAL_THRESHOLD=0.8` -> `layers.eval.threshold: 0.8`

## Development

Run with cargo:
```bash
cargo run --package loom-cli -- run datasets/samples.json -c configs/eval.yaml
cargo run --package loom-cli -- run datasets/samples.json -c configs/eval.yaml -v
```

## Navigation

[<- Back to Libraries](../README.md)
