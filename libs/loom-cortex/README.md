# loom-cortex

A Rust crate providing a unified, serializable wrapper around [rust-bert](https://github.com/guillaume-be/rust-bert) for configuring and managing NLP transformer models.

## Overview

`loom-cortex` serves as an abstraction layer that makes it easier to work with different types of NLP pipelines in a type-safe, configurable manner. It supports serialization for defining pipelines in configuration files.

## Features

- **Serializable Configuration** - Define NLP pipelines in JSON/TOML configuration files
- **Flexible Model Loading** - Support for HuggingFace defaults, custom paths, local directories, and remote URLs
- **Device Management** - Flexible GPU/CPU/MPS device selection
- **25+ Model Architectures** - Choose from BERT, GPT-2, T5, BART, and many more
- **13 NLP Task Types** - Conversation, summarization, translation, embeddings, and more
- **Type Safety** - Compile-time guarantees on model and config compatibility

## Supported Tasks

| Task | Description |
|------|-------------|
| `Conversation` | Multi-turn dialogue models |
| `MaskedLanguage` | Masked token prediction |
| `Ner` | Named Entity Recognition |
| `PosTagging` | Part-of-Speech tagging |
| `QuestionAnswering` | Extract answers from passages |
| `SentenceEmbeddings` | Convert text to vector embeddings |
| `Sentiment` | Classify emotional tone |
| `SequenceClassification` | Classify entire sequences |
| `Summarization` | Generate text summaries |
| `TextGeneration` | Generate new text |
| `TokenClassification` | Classify individual tokens |
| `Translation` | Translate between languages |
| `ZeroShotClassification` | Classify without training examples |

## Supported Model Architectures

### Encoder-Only
BERT, DistilBERT, DeBERTa, RoBERTa, XLM-RoBERTa, ELECTRA, MobileBERT, Albert, Longformer, FNet

### Decoder-Only
GPT-2, GPT-J, OpenAI-GPT, Reformer, GPT-Neo

### Encoder-Decoder
BART, MBart, T5, LongT5, Marian, Pegasus, ProphetNet, M2M-100, NLLB

## Usage

### Basic Configuration

```rust
use loom_cortex::{CortexModelConfig, CortexDevice, CortexModelSource};

// Create a text generation config
let config = CortexModelConfig::TextGeneration(
    CortexGenerationConfigBuilder::default()
        .device(CortexDevice::CudaIfAvailable)
        .source(CortexModelSource::Default)
        .build()
        .unwrap()
);

// Build the model
let model = config.build()?;
```

### Device Selection

```rust
use loom_cortex::CortexDevice;

// Automatic GPU selection (default)
let device = CortexDevice::CudaIfAvailable;

// Force CPU
let device = CortexDevice::Cpu;

// Specific CUDA device
let device = CortexDevice::Cuda(0);

// Apple Metal
let device = CortexDevice::Mps;
```

### Custom Model Loading

```rust
use loom_cortex::{CortexModelSource, CortexResource};

// Load from HuggingFace defaults
let source = CortexModelSource::Default;

// Load from local directory
let source = CortexModelSource::LocalDir {
    path: "/path/to/model".into(),
};

// Load with custom paths
let source = CortexModelSource::Custom {
    model: CortexResource::Local("/path/to/model.ot".into()),
    config: CortexResource::Local("/path/to/config.json".into()),
    vocab: CortexResource::Local("/path/to/vocab.txt".into()),
    merges: None,
};
```

### Sentence Embeddings

```rust
use loom_cortex::{
    CortexSentenceEmbeddingsConfig,
    CortexSentenceEmbeddingsModelType,
    CortexDevice,
};

let config = CortexSentenceEmbeddingsConfigBuilder::default()
    .model_type(CortexSentenceEmbeddingsModelType::AllMiniLmL12V2)
    .device(CortexDevice::CudaIfAvailable)
    .build()
    .unwrap();
```

## Scoring Abstractions

The `bench` module provides ML-specific abstractions for text scoring:

### Scorer Traits

```rust
use loom_cortex::bench::{Scorer, AsyncScorer, BatchScorer, ScorerOutput, Decision};

// Synchronous scoring
pub trait Scorer: Send + Sync {
    fn score(&self, text: &str) -> ScorerOutput;
}

// Async scoring
#[async_trait]
pub trait AsyncScorer: Send + Sync {
    async fn score(&self, text: &str) -> ScorerOutput;
}

// Batch inference optimization
pub trait BatchScorer: Send + Sync {
    fn score_batch(&self, texts: &[&str]) -> Vec<ScorerOutput>;
}
```

### Decision Enum

```rust
use loom_cortex::bench::Decision;

let decision = Decision::Accept;  // or Decision::Reject
```

### Platt Calibration

Types for Platt scaling probability calibration:

```rust
use loom_cortex::bench::platt::{PlattParams, PlattTrainingResult};
```

> **Note:** Operational types (datasets, results, runners) are in `loom_eval`.

## Module Structure

```
loom-cortex/
├── src/
│   ├── lib.rs              # Re-exports all modules
│   ├── model.rs            # CortexModel enum (13 task types)
│   ├── model_type.rs       # CortexModelType enum (25+ architectures)
│   ├── device.rs           # CortexDevice enum
│   ├── resource.rs         # Resource loading configuration
│   ├── bench/
│   │   ├── mod.rs          # Benchmark module exports
│   │   ├── scorer.rs       # Scorer, AsyncScorer, BatchScorer traits
│   │   ├── decision.rs     # Decision enum
│   │   └── platt/          # Platt calibration types
│   └── config/
│       ├── mod.rs
│       ├── model_config.rs # CortexModelConfig dispatcher
│       └── ...             # Task-specific configs
```

## Dependencies

- [rust-bert](https://github.com/guillaume-be/rust-bert) - Pre-trained transformer models
- [tch](https://github.com/LaurentMazare/tch-rs) - PyTorch bindings for Rust
- [serde](https://serde.rs/) - Serialization framework

## License

See the repository root for license information.

## Navigation

[← Back to Libraries](../README.md)
