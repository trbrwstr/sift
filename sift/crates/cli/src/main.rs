mod commands;

use clap::{Parser, Subcommand};
use commands::{analyze, stats};

#[derive(Parser)]
#[command(name = "logforge")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        file: String,

        #[arg(short, long, default_value = "plain")]
        format: String,

        #[arg(short, long)]
        filter: Option<String>,

        #[arg(short, long, default_value = "stdout")]
        output: String,
    },
    Stats {
        file: String,

        #[arg(short, long, default_value = "plain")]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Analyze {
            file,
            format,
            filter,
            output,
        } => analyze::run(analyze::AnalyzeArgs {
            file,
            format,
            filter,
            output,
        }),
        Commands::Stats { file, format } => {
            stats::run(stats::StatsArgs { file, format })
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}