use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use loom::io::path::{FilePath, Path};
use loom::runtime::{bench, score::ScoreConfig};

use super::build_runtime;
use crate::widgets::{self, Widget};

pub async fn exec(
    path: &PathBuf,
    config_path: &PathBuf,
    verbose: bool,
    concurrency: usize,
    batch_size: usize,
) {
    println!("Loading dataset from {:?}...", path);

    let runtime = build_runtime();
    let file_path = Path::File(FilePath::from(path.clone()));
    let dataset: bench::BenchDataset = match runtime.load("file_system", &file_path).await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error loading dataset: {}", e);
            std::process::exit(1);
        }
    };

    println!("Loaded {} samples", dataset.samples.len());
    println!("Loading config from {:?}...", config_path);

    let config_file_path = Path::File(FilePath::from(config_path.clone()));
    let config: ScoreConfig = match runtime.load("file_system", &config_file_path).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    println!("Building scorer (this may download model files on first run)...");

    // Build scorer in blocking task to avoid tokio runtime conflict with rust-bert
    let scorer = match tokio::task::spawn_blocking(move || config.build())
        .await
        .expect("spawn_blocking failed")
    {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error building scorer: {}", e);
            std::process::exit(1);
        }
    };

    if batch_size > 1 {
        println!("\nRunning benchmark with batch size {}...\n", batch_size);
    } else {
        println!(
            "\nRunning benchmark with {} parallel workers...\n",
            concurrency
        );
    }

    let scorer = Arc::new(Mutex::new(scorer));
    let total = dataset.samples.len();
    let config = bench::AsyncRunConfig {
        concurrency,
        batch_size: Some(batch_size),
    };

    let progress_callback = |p: bench::Progress| {
        let status = if p.correct { '✓' } else { '✗' };
        widgets::ProgressBar::new()
            .total(p.total)
            .current(p.current)
            .message(&p.sample_id)
            .status(status)
            .render()
            .write();
    };

    let result = if batch_size > 1 {
        bench::run_batch_async_with_config(&dataset, scorer, config, progress_callback).await
    } else {
        bench::run_async_with_config(&dataset, scorer, config, progress_callback).await
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

    if verbose {
        println!("\n=== Per-Category Results ===\n");
        let mut categories: Vec<_> = result.per_category.iter().collect();
        categories.sort_by_key(|(cat, _)| format!("{:?}", cat));

        for (category, cat_result) in categories {
            let cat_metrics = metrics.per_category.get(category);
            let accuracy = cat_metrics.map(|m| m.accuracy).unwrap_or(0.0);
            println!(
                "{:12} {:3}/{:3} ({:.1}%)",
                format!("{:?}", category),
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
}
