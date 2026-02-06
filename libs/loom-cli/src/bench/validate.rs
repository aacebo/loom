use std::path::PathBuf;

use loom::io::path::{FilePath, Path};
use loom::runtime::bench;

use super::build_runtime;

pub async fn exec(path: &PathBuf) {
    println!("Validating dataset at {:?}...", path);

    let runtime = build_runtime();
    let file_path = Path::File(FilePath::from(path.clone()));

    let dataset: bench::BenchDataset = match runtime.load("file_system", &file_path).await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error loading dataset: {}", e);
            std::process::exit(1);
        }
    };

    let errors = dataset.validate();

    if errors.is_empty() {
        println!("✓ Dataset is valid ({} samples)", dataset.samples.len());
    } else {
        println!("✗ Found {} validation error(s):\n", errors.len());
        for error in &errors {
            println!("  - {}", error);
        }
        std::process::exit(1);
    }
}
