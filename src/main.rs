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
fn main() {
    let dt_str = "2024-12-10 10:23:45";
    let time_stamp: DateTime = dt_str.parse().unwrap();
    println!("timestamp: {}", time_stamp);
    
    let time_stamp2: DateTime = "2024-02-12 10:12:2".parse().unwrap();
    let time_stamp3: DateTime = "2023-02-12 10:12:2".parse().unwrap();

    println!("t2: {}",time_stamp2);
    let mut v = vec![time_stamp,time_stamp2,time_stamp3];

    v.sort();

    for t in &v {
        println!("{}", t);
    }

    let line = "2024-01-02 fkas";
    let source_file = Path::new("src/log_file.txt");

    match log_entry::parse_log_line(line, source_file) {
        Ok(entry) => println!("Parsed: {:?}", entry),
        Err(e) => println!("Parse failed: {:?}", e),
    }

    // *************** Statistics ********

    let source_file = Path::new("src/log_file.txt");

    let lines = vec![
        "2024-01-15 7:23:45 [ERROR] storage: Failed to mount filesystem /dev/sda1",
        "2024-06-15 8:23:46 [INFO] network: Connection established to 192.168.1.100",
        "2024-01-15 9:2347 [WARN] storage: Multiple failed login attempts from user admin",
        "2024-11-15 8:24:01 [INFO] scheduler: Task queue processing started",
        "2024-10-15 10:24:02 [FATAL] memory: Garbage collection cycle completed",
        "2024-09-15 12:24:05 [FATAL] kernel: Out of memory - system unstable",
    ];

    let mut entries: Vec<LogEntry> = Vec::new();
    let mut errors: Vec<ParseError> = Vec::new();

    for line in lines {
        match parse_log_line(line, source_file) {
            Ok(entry) => entries.push(entry),
            Err(err) => errors.push(err)
        }
    }

    let stats = Statistics::from_entries(&entries);
    println!("stats:- {:?}",stats);
    println!("Total entries = {}", stats.total_entries);
    println!("{}", stats.error_rate);
    println!("{}",stats.error_count);
    println!("Error: {:?}",errors);


    // ********* Log Analyzer File ************
    println!("****** Log Analyzer - single file *********");
    let mut analyzer:LogAnalyzer = LogAnalyzer::new();
    let path:&Path = Path::new("src/logs/all.log");

    let count = analyzer.process_file(path).unwrap();
    println!("count: {}",count);
    let stats = analyzer.statistics();
    println!("Stats: {:?}",stats);
    let errors = analyzer.parse_errors();
    println!("errors: {:?}",errors);

    let entries = analyzer.entries();

    println!("Entries: {:?}",entries);

    // ****** Directory *********
    println!("******** Directory ******");
    let mut analyzer = LogAnalyzer::new();
    let path: &Path = Path::new("src/logs");

    match analyzer.process_directory(path) {
        Ok(count) => {
            println!("Processed {} entries", count);
            println!("Entries: {}", analyzer.entries().len());
            println!("Errors: {}", analyzer.parse_errors().len());
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
    let en = analyzer.entries();
    println!("{:?}", en);
}
