# loom-cli

Command-line interface for the Loom scoring runtime.

## Installation

```bash
cargo build --package loom-cli
```

## Usage

```bash
loom<command> [options]
```

## Commands

### `bench` - Benchmark Operations

#### `bench run` - Run benchmark against a dataset

```bash
loombench run <path> [options]

Options:
  -t, --threshold <THRESHOLD>  Base threshold for scoring [default: 0.75]
  -d, --dynamic                Enable dynamic thresholds based on text length
```

Example:
```bash
loombench run datasets/dataset.json
loombench run datasets/dataset.json --threshold 0.80 --dynamic
```

#### `bench validate` - Validate a benchmark dataset

```bash
loombench validate <path>
```

Checks for:
- Valid JSON structure
- Required fields present
- Valid label names
- Valid decision values (accept/reject)
- No duplicate sample IDs

#### `bench coverage` - Show label coverage for a dataset

```bash
loombench coverage <path>
```

Displays:
- Total samples and accept/reject breakdown
- Samples per category (target: 50 each)
- Samples per label (target: 3+ each)
- Missing labels (if any)

## Development

Run with cargo:
```bash
cargo run --package loom-cli -- bench run datasets/dataset.json
```
