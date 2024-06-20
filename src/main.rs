use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Data,
    Backtest,
    Optimize,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Data => {
            println!("Data command");
        }
        Commands::Backtest => {
            println!("Backtest command");
        }
        Commands::Optimize => {
            println!("Optimize command");
        }
    }
}
