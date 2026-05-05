use logforge_core::engine::run_pipeline;
use logforge_formats::{
    json::JsonParser,
    nginx::NginxParser,
    plain::PlainParser,
};
use logforge_output::{stdout, table};

pub struct AnalyzeArgs {
    pub file: String,
    pub format: String,
    pub filter: Option<String>,
    pub output: String,
}

pub fn run(args: AnalyzeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let agg = match args.format.as_str() {
        "json" => run_pipeline(&args.file, JsonParser, args.filter)?,
        "nginx" => run_pipeline(&args.file, NginxParser::new(), args.filter)?,
        _ => run_pipeline(&args.file, PlainParser, args.filter)?,
    };

    match args.output.as_str() {
        "table" => table::print_table(&agg),
        _ => stdout::print_summary(&agg),
    }

    Ok(())
}