use std::io::stdout;
use std::path::PathBuf;

use crossterm::ExecutableCommand;
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use loom::io::path::{FilePath, Path};
use loom::runtime::bench;

use super::build_runtime;
use crate::widgets::{self, Widget};

pub async fn exec(path: &PathBuf) {
    widgets::Spinner::new()
        .message(format!("Validating dataset at {:?}...", path))
        .render()
        .write();

    let runtime = build_runtime();
    let file_path = Path::File(FilePath::from(path.clone()));
    let dataset: bench::BenchDataset = match runtime.load("file_system", &file_path).await {
        Ok(d) => d,
        Err(e) => {
            widgets::Spinner::clear();
            eprintln!("Error loading dataset: {}", e);
            std::process::exit(1);
        }
    };

    widgets::Spinner::clear();

    let errors = dataset.validate();
    let mut stdout = stdout();

    if errors.is_empty() {
        let _ = stdout.execute(SetForegroundColor(Color::Green));
        print!("✓ ");
        let _ = stdout.execute(ResetColor);
        println!("Dataset is valid ({} samples)", dataset.samples.len());
    } else {
        let _ = stdout.execute(SetForegroundColor(Color::Red));
        print!("✗ ");
        let _ = stdout.execute(ResetColor);
        println!("Found {} validation error(s):\n", errors.len());
        for error in &errors {
            println!("  - {}", error);
        }
        std::process::exit(1);
    }
}
