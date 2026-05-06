use sift_core::engine::run_pipeline;
use sift_formats::make_parser;
use sift_output::{json, stdout, table};

pub struct AnalyzeArgs {
    pub file: String,
    pub format: String,
    pub filter: Option<String>,
    pub output: String,
}

pub fn run(args: AnalyzeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let agg = run_pipeline(&args.file, make_parser(&args.format), args.filter)?;

    match args.output.as_str() {
        "table" => table::print_table(&agg),
        "json" => json::print_json(&agg),
        _ => stdout::print_summary(&agg),
    }

    Ok(())
}
