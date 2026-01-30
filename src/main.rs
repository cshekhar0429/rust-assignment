use std::path::PathBuf;
mod date_time;
mod log_level;
mod log_entry;
mod statistics_aggregator;
mod log_analyzer;
use log_analyzer::LogAnalyzer;

mod report;
use report::{print_report, print_report_json};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    paths: Vec<PathBuf>,
    #[arg(long, default_value = "text")]
    format: String,
}

fn main() {
 let cli = Cli::parse();

    if cli.paths.is_empty() {
        println!("No File or Directory provided");
        return;
    }

    let mut analyzer = LogAnalyzer::new();

    for path in &cli.paths {
        let result = if path.is_file() {
            analyzer.process_file(path)
        } else if path.is_dir() {
            analyzer.process_directory(path)
        } else {
            println!("Invalid path: {:?}", path);
            continue;
        };

        if let Err(e) = result {
            println!("Error processing {:?}: {:?}", path, e);
        }
    }

    let stats = analyzer.statistics();
    let error_count = analyzer.parse_errors().len();

    println!("Errors in files: {:?}", analyzer.parse_errors());

    if cli.format == "json" {
        print_report_json(&stats, error_count);
    } else {
        print_report(&stats, error_count);
    }
}
