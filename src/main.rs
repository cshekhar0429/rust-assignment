use std::path::Path;
mod date_time;
use date_time::DateTime;

mod log_level;
use log_level::LogLevel;

mod log_entry;
use log_entry::LogEntry;
use log_entry::parse_log_line;
use log_entry::ParseError;

mod statistics_aggregator;
use statistics_aggregator::Statistics;

mod log_analyzer;
use log_analyzer::LogAnalyzer;
use log_analyzer::AnalyzerError;

mod report;
use report::print_report;

fn main() {

    println!("******** Directory ******");
    let mut analyzer = LogAnalyzer::new();
    let path: &Path = Path::new("src/logs");

    let count = analyzer.process_directory(path).unwrap();
    let error_count = analyzer.parse_errors().len();
    let stats = analyzer.statistics();
    print_report(&stats, error_count);

}
