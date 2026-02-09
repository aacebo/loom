use clap::{Parser, Subcommand};

mod commands;
pub mod widgets;

use commands::RunCommand;

/// Loom scoring engine CLI
///
/// Evaluate, validate, and train ML-based content scoring models.
#[derive(Parser)]
#[command(name = "loom")]
#[command(version, author)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run evaluation against a dataset
    Run(RunCommand),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(cmd) => cmd.exec().await,
    }
}
