use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "logforge")]
#[command(about = "Blazing-fast log analyzer in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Analyze {
        file: String,

        #[arg(short, long, default_value = "plain")]
        pub format: String,

        #[arg(short, long)]
        pub filter: Option<String>,

        #[arg(short, long, default_value = "stdout")]
        pub output: String,

        #[arg(long)]
        pub top: Option<usize>,
    },

    Stats {
        file: String,

        #[arg(short, long, default_value = "plain")]
        pub format: String,
    },
}