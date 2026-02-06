use clap::{Parser, Subcommand};

mod bench;
pub mod widgets;

#[derive(Parser)]
#[command(name = "loom")]
#[command(about = "Loom scoring engine CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Benchmark operations
    Bench {
        #[command(subcommand)]
        action: bench::BenchAction,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bench { action } => bench::run(action).await,
    }
}
