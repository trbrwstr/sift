use sift_core::engine::run_pipeline;
use sift_formats::{make_parser, LogFormat};
use sift_output::{json, stdout, table};
use crate::OutputFormat;

pub struct AnalyzeArgs {
    pub file: String,
    pub format: LogFormat,
    pub filter: Option<String>,
    pub output: OutputFormat,
    pub top: usize,
}

pub fn run(args: AnalyzeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let agg = run_pipeline(&args.file, make_parser(&args.format), args.filter)?;

    match args.output {
        OutputFormat::Table  => table::print_table(&agg),
        OutputFormat::Json   => json::print_json(&agg),
        OutputFormat::Stdout => stdout::print_summary(&agg, args.top),
    }

    Ok(())
}
