use sift_core::engine::run_pipeline;
use sift_formats::{make_parser, LogFormat};
use sift_output::stdout;

pub struct StatsArgs {
    pub file: String,
    pub format: LogFormat,
}

pub fn run(args: StatsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let agg = run_pipeline(&args.file, make_parser(&args.format), None)?;
    stdout::print_summary(&agg, 5);
    Ok(())
}
