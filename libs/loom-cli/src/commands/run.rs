use std::path::PathBuf;

use clap::Args;
use loom::core::{Format, ident_path};
use loom::eval::{EvalConfig, EvalLayer, EvalOutput, EvalResult, SampleDataset};
use loom::io::path::{FilePath, Path};
use loom::runtime::{
    Emitter, FileSystemSource, JsonCodec, LoomConfig, Runtime, Signal, TomlCodec, YamlCodec,
};

use super::{load_config, resolve_output_path};
use crate::widgets::{self, Widget};

/// Signal emitter that displays progress on stdout.
struct ProgressEmitter;

impl Emitter for ProgressEmitter {
    fn emit(&self, signal: Signal) {
        if signal.name() == "eval.scored" {
            let attrs = signal.attributes();
            let score = attrs.get("score").and_then(|v| v.as_float()).unwrap_or(0.0);

            widgets::ProgressBar::new()
                .message(&format!("{:.2}", score))
                .render()
                .write();
        }
    }
}

/// Run evaluation against a dataset
#[derive(Debug, Args)]
pub struct RunCommand {
    /// Path to the dataset JSON file
    pub path: PathBuf,

    /// Path to config file (YAML/JSON/TOML)
    #[arg(short, long)]
    pub config: PathBuf,

    /// Output directory for results (default: input file's directory)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Show detailed per-category and per-label results
    #[arg(short, long)]
    pub verbose: bool,
}

impl RunCommand {
    pub async fn exec(self) {
        println!("Loading config from {:?}...", self.config);

        let config = match load_config(self.config.to_str().unwrap_or_default()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error loading config: {}", e);
                std::process::exit(1);
            }
        };

        // Read eval config for threshold calculation (before config is moved)
        let loom_config: LoomConfig = config.root_section().bind().unwrap_or_default();
        let eval_config: Option<EvalConfig> = {
            let eval_path = ident_path!("layers.eval");
            let section = config.get_section(&eval_path);
            section.bind().ok()
        };

        println!("Building runtime (this may download model files on first run)...");

        // Build eval layer in spawn_blocking (rust-bert model download conflicts with tokio)
        let eval_layer =
            match tokio::task::spawn_blocking(move || EvalLayer::from_config(&config)).await {
                Ok(Ok(layer)) => layer,
                Ok(Err(e)) => {
                    eprintln!("Error building eval layer: {}", e);
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error building eval layer: {}", e);
                    std::process::exit(1);
                }
            };

        // Build runtime with externally-supplied layer
        let runtime = Runtime::new()
            .source(FileSystemSource::builder().build())
            .codec(JsonCodec::new())
            .codec(YamlCodec::new())
            .codec(TomlCodec::new())
            .layer(eval_layer)
            .emitter(ProgressEmitter)
            .build();

        let output_dir = self.output.as_ref().or(loom_config.output.as_ref());
        let output_path =
            resolve_output_path(&self.path, output_dir.map(|p| p.as_path()), "results.json");

        println!("Loading dataset...");

        let file_path = FilePath::from(self.path.clone()).into();
        let dataset: SampleDataset = match runtime.load("file_system", &file_path).await {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error loading dataset: {}", e);
                std::process::exit(1);
            }
        };

        let eval_start = std::time::Instant::now();
        let total = dataset.samples.len();
        let mut result = EvalResult::new();

        println!("Running evaluation on {} samples...\n", total);

        for sample in &dataset.samples {
            let output_value = match runtime.execute(sample.text.clone()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error executing pipeline for sample {}: {}", sample.id, e);
                    std::process::exit(1);
                }
            };

            let output: EvalOutput = match output_value.try_into() {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("Error converting output for sample {}: {}", sample.id, e);
                    std::process::exit(1);
                }
            };

            let threshold = eval_config
                .as_ref()
                .map(|c| c.threshold_of(sample.text.len()))
                .unwrap_or(0.75);

            result = result.merge(output.to_result(sample, threshold));
        }

        let elapsed = eval_start.elapsed();
        result.elapsed_ms = elapsed.as_millis() as i64;
        result.throughput = if elapsed.as_secs_f32() > 0.0 {
            total as f32 / elapsed.as_secs_f32()
        } else {
            0.0
        };

        // Clear the progress line
        widgets::ProgressBar::clear();
        println!("Completed {} samples\n", total);

        // Compute metrics from raw counts
        let metrics = result.metrics();

        // Display prominent score summary
        let score_out_of_100 = (metrics.accuracy * 100.0).round() as u32;
        println!("========================================");
        println!(
            "  SCORE: {}/100 ({:.1}%)",
            score_out_of_100,
            metrics.accuracy * 100.0
        );
        println!("========================================\n");

        println!("=== Benchmark Results ===\n");
        println!("Total samples: {}", result.total);
        println!(
            "Correct:       {} ({:.1}%)",
            result.correct,
            metrics.accuracy * 100.0
        );
        println!();
        println!("Precision: {:.3}", metrics.precision);
        println!("Recall:    {:.3}", metrics.recall);
        println!("F1 Score:  {:.3}", metrics.f1);

        if self.verbose {
            println!("\n=== Per-Category Results ===\n");
            let mut categories: Vec<_> = result.per_category.iter().collect();
            categories.sort_by_key(|(cat, _)| cat.as_str());

            for (category, cat_result) in categories {
                let cat_metrics = metrics.per_category.get(category);
                let accuracy = cat_metrics.map(|m| m.accuracy).unwrap_or(0.0);
                println!(
                    "{:20} {:3}/{:3} ({:.1}%)",
                    category,
                    cat_result.correct,
                    cat_result.total,
                    accuracy * 100.0
                );
            }

            println!("\n=== Per-Label Results ===\n");

            let mut labels: Vec<_> = result.per_label.iter().collect();
            labels.sort_by_key(|(label, _)| label.as_str());

            let mut table = widgets::Table::new().headers(vec![
                "Label", "Expect", "Detect", "TP", "Prec", "Recall", "F1",
            ]);

            for (label, label_result) in labels {
                if label_result.expected_count > 0 || label_result.detected_count > 0 {
                    let label_metrics = metrics.per_label.get(label);
                    let (precision, recall, f1) = label_metrics
                        .map(|m| (m.precision, m.recall, m.f1))
                        .unwrap_or((0.0, 0.0, 0.0));
                    table = table.row(vec![
                        label.to_string(),
                        label_result.expected_count.to_string(),
                        label_result.detected_count.to_string(),
                        label_result.true_positives.to_string(),
                        format!("{:.3}", precision),
                        format!("{:.3}", recall),
                        format!("{:.3}", f1),
                    ]);
                }
            }

            print!("{}", table);

            // Show misclassified samples
            let incorrect: Vec<_> = result
                .sample_results
                .iter()
                .filter(|s| !s.correct)
                .collect();

            if !incorrect.is_empty() {
                println!("\n=== Misclassified Samples ({}) ===\n", incorrect.len());
                for sample in incorrect.iter().take(10) {
                    println!("ID: {}", sample.id);
                    println!(
                        "  Expected: {:?}, Actual: {:?}",
                        sample.expected_decision, sample.actual_decision
                    );
                    println!("  Score: {:.3}", sample.score);
                    println!("  Expected labels: {:?}", sample.expected_labels);
                    println!("  Detected labels: {:?}", sample.detected_labels);
                    println!();
                }
                if incorrect.len() > 10 {
                    println!("... and {} more", incorrect.len() - 10);
                }
            }
        }

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Error creating output directory: {}", e);
                std::process::exit(1);
            }
        }

        // Write results to output file
        let file_path = Path::File(FilePath::from(output_path.clone()));
        if let Err(e) = runtime
            .save("file_system", &file_path, &result, Format::Json)
            .await
        {
            eprintln!("Error writing output file: {}", e);
            std::process::exit(1);
        }

        println!("\nResults written to {:?}", output_path);
    }
}
