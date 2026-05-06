mod commands;

use clap::{Parser, Subcommand, ValueEnum};
use commands::{analyze, stats, tail};

#[derive(Clone, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Stdout,
    Table,
    Json,
}

#[derive(Parser)]
#[command(name = "sift", about = "Fast log analysis for large files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a log file and display a summary
    Analyze {
        file: String,

        #[arg(short, long, default_value = "plain",
              help = "Log format: plain, json, nginx")]
        format: String,

        #[arg(long, help = "Filter expression, e.g. 'status>=500 AND level=ERROR'")]
        filter: Option<String>,

        #[arg(long, value_enum, default_value_t = OutputFormat::Stdout,
              help = "Output style: stdout, table, json")]
        output: OutputFormat,

        #[arg(long, default_value_t = 5,
              help = "Number of top messages to display")]
        top: usize,
    },
    /// Print a quick entry-count and level breakdown
    Stats {
        file: String,

        #[arg(short, long, default_value = "plain")]
        format: String,
    },
    /// Stream new lines appended to a file (like tail -f)
    Tail {
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Analyze { file, format, filter, output, top } => {
            analyze::run(analyze::AnalyzeArgs { file, format, filter, output, top })
        }
        Commands::Stats { file, format } => {
            stats::run(stats::StatsArgs { file, format })
        }
        Commands::Tail { file } => {
            tail::run(&file).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
