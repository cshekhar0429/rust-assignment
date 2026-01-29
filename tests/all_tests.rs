use std::path::Path;
use log_analyzer::date_time::DateTime;
use log_analyzer::log_level::LogLevel;
use log_analyzer::log_entry::{parse_log_line, LogEntry};
use log_analyzer::statistics_aggregator::Statistics;
use log_analyzer::log_analyzer::LogAnalyzer;

// ---------------- DATETIME TESTS ----------------

#[test]
fn datetime_parse_valid() {
    let dt: Result<DateTime, _> = "2024-01-15 10:23:45".parse();
    assert!(dt.is_ok());
}

#[test]
fn datetime_reject_invalid_month() {
    assert!("2024-00-10 10:00:00".parse::<DateTime>().is_err());
    assert!("2024-13-10 10:00:00".parse::<DateTime>().is_err());
}

#[test]
fn datetime_reject_invalid_hour() {
    assert!("2024-01-10 24:00:00".parse::<DateTime>().is_err());
    assert!("2024-01-10 25:00:00".parse::<DateTime>().is_err());
}

#[test]
fn datetime_compare_two() {
    let a: DateTime = "2024-01-01 10:00:00".parse().unwrap();
    let b: DateTime = "2024-01-01 11:00:00".parse().unwrap();

    assert!(a < b);
}

// ---------------- LOG LEVEL TESTS ----------------

#[test]
fn loglevel_parse_valid() {
    assert!("INFO".parse::<LogLevel>().is_ok());
    assert!("WARN".parse::<LogLevel>().is_ok());
    assert!("ERROR".parse::<LogLevel>().is_ok());
    assert!("FATAL".parse::<LogLevel>().is_ok());
}

#[test]
fn loglevel_case_insensitive() {
    let a: LogLevel = "info".parse().unwrap();
    let b: LogLevel = "INFO".parse().unwrap();
    assert_eq!(a, b);
}

#[test]
fn loglevel_ordering() {
    let info: LogLevel = "INFO".parse().unwrap();
    let error: LogLevel = "ERROR".parse().unwrap();
    assert!(error > info);
}

// ---------------- PARSER TESTS ----------------

#[test]
fn parser_valid_line() {
    let line = "2024-01-15 10:23:45 [INFO] storage: Started";
    let path = Path::new("dummy.log");
    let result = parse_log_line(line, path, 1);
    assert!(result.is_ok());
}

#[test]
fn parser_extra_whitespace() {
    let line = " 2024-01-15 10:23:45   [INFO]  storage: Started ";
    let path = Path::new("dummy.log");
    let result = parse_log_line(line, path, 1);
    assert!(result.is_ok());
}

#[test]
fn parser_malformed_line() {
    let line = "123 dummy";
    let path = Path::new("dummy.log");
    let result = parse_log_line(line, path, 1);
    assert!(result.is_err());
}

// ---------------- STATISTICS TESTS ----------------

#[test]
fn statistics_sample_entries() {
    let path = Path::new("dummy.log");

    let e1 = parse_log_line(
        "2024-01-15 10:23:45 [INFO] storage: Started",
        path,
        1
    )
    .unwrap();

    let e2 = parse_log_line(
        "2024-01-15 11:23:45 [ERROR] network: Failed",
        path,
        1
    )
    .unwrap();

    let entries = vec![e1, e2];
    let stats = Statistics::from_entries(&entries);

    assert_eq!(stats.total_entries, 2);
}

#[test]
fn statistics_empty_input() {
    let entries: Vec<LogEntry> = vec![];
    let stats = Statistics::from_entries(&entries);
    assert_eq!(stats.total_entries, 0);
}

#[test]
fn statistics_percentages() {
    let path = Path::new("dummy.log");

    let e1 = parse_log_line(
        "2024-01-15 10:23:45 [ERROR] storage: Failed",
        path,
        1,
    )
    .unwrap();

    let e2 = parse_log_line(
        "2024-01-15 11:23:45 [INFO] network: OK",
        path,
        1,
    )
    .unwrap();

    let entries = vec![e1, e2];
    let stats = Statistics::from_entries(&entries);

    assert!(stats.error_rate > 0.4 && stats.error_rate < 0.6);
}

// ---------------- FILE PROCESSING TESTS ----------------
#[test]
fn file_process_valid_file() {
    let mut analyzer = LogAnalyzer::new();
    let path = Path::new("src/logs/all.log");

    let result = analyzer.process_file(path);
    assert!(result.is_ok());
}

#[test]
fn file_missing_file() {
    let mut analyzer = LogAnalyzer::new();
    let path = Path::new("abc.log");

    let result = analyzer.process_file(path);
    assert!(result.is_err());
}

#[test]
fn file_skip_malformed_lines() {
    let mut analyzer = LogAnalyzer::new();
    let path = Path::new("src/logs");

    let result = analyzer.process_directory(path);
    assert!(result.is_ok());
}
