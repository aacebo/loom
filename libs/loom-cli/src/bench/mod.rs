use std::path::PathBuf;

use clap::Subcommand;
use loom::runtime::{FileSystemSource, JsonCodec, Runtime, TomlCodec, YamlCodec};

mod cov;
mod run;
mod score;
mod train;
mod validate;

/// Build a Runtime configured with standard sources and codecs.
pub fn build_runtime() -> Runtime {
    Runtime::new()
        .source(FileSystemSource::builder().build())
        .codec(JsonCodec::new())
        .codec(YamlCodec::new())
        .codec(TomlCodec::new())
        .build()
}

#[derive(Subcommand)]
pub enum BenchAction {
    /// Run benchmark against a dataset
    Run {
        /// Path to the benchmark dataset JSON file
        path: PathBuf,
        /// Path to config file (YAML/JSON/TOML)
        #[arg(short, long)]
        config: PathBuf,
        /// Show detailed per-category and per-label results
        #[arg(short, long)]
        verbose: bool,
        /// Number of parallel inference workers (overrides config)
        #[arg(long)]
        concurrency: Option<usize>,
        /// Batch size for ML inference (overrides config)
        #[arg(long)]
        batch_size: Option<usize>,
        /// Fail if samples have categories/labels not in config (overrides config)
        #[arg(long)]
        strict: Option<bool>,
    },
    /// Validate a benchmark dataset
    Validate {
        /// Path to the benchmark dataset JSON file
        path: PathBuf,
        /// Path to score config file (YAML/JSON/TOML) for category/label validation
        #[arg(short, long)]
        config: Option<PathBuf>,
        /// Fail if samples have categories/labels not in config (default: report errors)
        #[arg(long)]
        strict: bool,
    },
    /// Show label coverage for a dataset
    Coverage {
        /// Path to the benchmark dataset JSON file
        path: PathBuf,
    },
    /// Extract raw scores for Platt calibration training
    Score {
        /// Path to the benchmark dataset JSON file
        path: PathBuf,
        /// Path to config file (YAML/JSON/TOML)
        #[arg(short, long)]
        config: PathBuf,
        /// Output path for results (overrides config)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Number of parallel inference workers (overrides config)
        #[arg(long)]
        concurrency: Option<usize>,
        /// Batch size for ML inference (overrides config)
        #[arg(long)]
        batch_size: Option<usize>,
        /// Fail if samples have categories/labels not in config (overrides config)
        #[arg(long)]
        strict: Option<bool>,
    },
    /// Train Platt calibration parameters from raw scores
    Train {
        /// Path to raw scores JSON (from extract-scores)
        path: PathBuf,
        /// Output path for trained parameters JSON
        #[arg(short, long)]
        output: PathBuf,
        /// Also output Rust code for label.rs
        #[arg(long)]
        code: bool,
    },
}

pub async fn run(action: BenchAction) {
    match action {
        BenchAction::Run {
            path,
            config,
            verbose,
            concurrency,
            batch_size,
            strict,
        } => run::exec(&path, &config, verbose, concurrency, batch_size, strict).await,
        BenchAction::Validate {
            path,
            config,
            strict,
        } => validate::exec(&path, config.as_ref(), strict).await,
        BenchAction::Coverage { path } => cov::exec(&path).await,
        BenchAction::Score {
            path,
            config,
            output,
            concurrency,
            batch_size,
            strict,
        } => {
            score::exec(
                &path,
                &config,
                output.as_ref(),
                concurrency,
                batch_size,
                strict,
            )
            .await
        }
        BenchAction::Train { path, output, code } => train::exec(&path, &output, code).await,
    }
}
