use std::path::PathBuf;

use loom::core::Format;
use loom::io::path::{FilePath, Path};
use loom::runtime::bench;

use super::build_runtime;

pub async fn exec(path: &PathBuf, output: &PathBuf, generate_rust: bool) {
    println!("Loading raw scores from {:?}...", path);

    let runtime = build_runtime();
    let file_path = Path::File(FilePath::from(path.clone()));

    let export: bench::RawScoreExport = match runtime.load("file_system", &file_path).await {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error loading file: {}", e);
            std::process::exit(1);
        }
    };

    println!("Loaded {} samples", export.samples.len());
    println!("\nTraining Platt parameters...");

    let result = bench::train_platt_params(&export);

    // Display results
    println!("\n=== Training Results ===\n");

    let mut sorted_labels: Vec<_> = result.params.iter().collect();
    sorted_labels.sort_by_key(|(k, _)| k.as_str());

    for (label, params) in &sorted_labels {
        let stats = result.metadata.samples_per_label.get(*label);
        let status = if let Some(s) = stats {
            if s.skipped {
                format!("SKIPPED (pos={}, neg={})", s.positive, s.negative)
            } else {
                format!("pos={}, neg={}", s.positive, s.negative)
            }
        } else {
            "".to_string()
        };
        println!(
            "{:20} a={:7.4}, b={:7.4}  [{}]",
            label, params.a, params.b, status
        );
    }

    // Write parameters to output file using runtime
    let output_path = Path::File(FilePath::from(output.clone()));
    if let Err(e) = runtime
        .save("file_system", &output_path, &result, Format::Json)
        .await
    {
        eprintln!("\nError writing output file: {}", e);
        std::process::exit(1);
    }

    println!("\nParameters written to {:?}", output);

    if generate_rust {
        let rust_code = bench::generate_rust_code(&result);
        println!("\n=== Rust Code ===\n");
        println!("{}", rust_code);
    }
}
