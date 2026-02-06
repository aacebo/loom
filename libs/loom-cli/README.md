# loom-cli

Command-line interface for the Loom scoring runtime.

## Installation

```bash
cargo build --package loom-cli
```

## Usage

```bash
loom-cli <command> [options]
```

## Commands

### `bench` - Benchmark Operations

#### `bench run` - Run benchmark against a dataset

```bash
loom-cli bench run <path> --config <config> [options]

Options:
  -c, --config <CONFIG>  Path to score config file (YAML/JSON/TOML)
  -v, --verbose          Show detailed per-category and per-label results
```

Example:
```bash
loom-cli bench run datasets/dataset.json --config configs/score.config.yaml
loom-cli bench run datasets/dataset.json --config configs/score.config.yaml -v
```

#### `bench validate` - Validate a benchmark dataset

```bash
loom-cli bench validate <path>
```

Checks for:
- Valid JSON structure
- Required fields present
- Valid label names
- Valid decision values (accept/reject)
- No duplicate sample IDs

#### `bench coverage` - Show label coverage for a dataset

```bash
loom-cli bench coverage <path>
```

Displays:
- Total samples and accept/reject breakdown
- Samples per category (target: 50 each)
- Samples per label (target: 3+ each)
- Missing labels (if any)

#### `bench score` - Extract raw scores for Platt calibration

```bash
loom-cli bench score <path> --config <config> --output <output>

Options:
  -c, --config <CONFIG>  Path to score config file (YAML/JSON/TOML)
  -o, --output <OUTPUT>  Output path for raw scores JSON
```

#### `bench train` - Train Platt calibration parameters

```bash
loom-cli bench train <path> --output <output> [options]

Options:
  -o, --output <OUTPUT>  Output path for trained parameters
  --code                 Also output Rust code for label.rs
```

## Development

Run with cargo:
```bash
cargo run --package loom-cli -- bench run datasets/dataset.json --config configs/score.config.yaml
```
