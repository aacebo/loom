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
        .message(format!("Analyzing coverage for {:?}...", path))
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

    let coverage = dataset.coverage();
    let mut stdout = stdout();

    println!("=== Dataset Coverage ===\n");
    println!("Total samples: {}", coverage.total_samples);
    println!(
        "Accept: {}, Reject: {}",
        coverage.accept_count, coverage.reject_count
    );

    println!("\n=== By Category ===\n");
    let mut categories: Vec<_> = coverage.samples_by_category.iter().collect();
    categories.sort_by_key(|(cat, _)| cat.as_str());

    for (cat, count) in categories {
        let target = 50;
        let (status, color) = if *count >= target {
            ("✓", Color::Green)
        } else {
            ("○", Color::Yellow)
        };
        let _ = stdout.execute(SetForegroundColor(color));
        print!("  {} ", status);
        let _ = stdout.execute(ResetColor);
        println!("{:20} {:3}/{}", cat, count, target);
    }

    println!("\n=== By Label ===\n");
    let mut labels: Vec<_> = coverage.samples_by_label.iter().collect();
    labels.sort_by_key(|(label, _)| label.as_str());

    for (label, count) in labels {
        let (status, color) = if *count >= 3 {
            ("✓", Color::Green)
        } else {
            ("○", Color::Yellow)
        };
        let _ = stdout.execute(SetForegroundColor(color));
        print!("  {} ", status);
        let _ = stdout.execute(ResetColor);
        println!("{:20} {}", label, count);
    }

    if !coverage.missing_labels.is_empty() {
        println!(
            "\n=== Missing Labels ({}) ===\n",
            coverage.missing_labels.len()
        );
        for label in &coverage.missing_labels {
            let _ = stdout.execute(SetForegroundColor(Color::Red));
            print!("  ✗ ");
            let _ = stdout.execute(ResetColor);
            println!("{}", label);
        }
    }
}
