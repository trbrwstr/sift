use logforge_core::engine::run_pipeline;
use logforge_formats::{
    json::JsonParser,
    nginx::NginxParser,
    plain::PlainParser,
};

pub struct StatsArgs {
    pub file: String,
    pub format: String,
}

pub fn run(args: StatsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let agg = match args.format.as_str() {
        "json" => run_pipeline(&args.file, JsonParser, None)?,
        "nginx" => run_pipeline(&args.file, NginxParser::new(), None)?,
        _ => run_pipeline(&args.file, PlainParser, None)?,
    };

    println!("\n=== Quick Stats ===");
    println!("Total entries: {}", agg.total);

    if !agg.levels.is_empty() {
        println!("\nLevels:");
        for (level, count) in &agg.levels {
            println!("  {:<10} {}", level, count);
        }
    }

    println!();
    Ok(())
}