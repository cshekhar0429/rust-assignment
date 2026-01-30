# rust-assignment

## Project Setup
    created a new project called log-analayzer and added to cargo.toml:
    [dependencies]
    thiserror = "1.0

## Implemented Parts

## 1. DateTime Parsing

Implemented parsing for the `DateTime` struct using the `FromStr` trait.
This means we can create a `DateTime` instance directly from a timestamp string using the `parse()` method, and it will automatically return a valid `DateTime` instance (or an error if the input is invalid).

Supported format:

Validation rules:

- Year: 1970 to 9999  
- Month: 1 to 12  
- Day: 1 to 31  
- Hour: 0 to 23  
- Minute: 0 to 59  
- Second: 0 to 59  

Also implemented:

- `Display` trait (to convert `DateTime` back into a string)
- `PartialOrd` and `Ord` (so timestamps can be sorted)

---

---
### 2. LogLevel Parsing

Implemented parsing for the `LogLevel` enum using the `FromStr` trait.
So we can create a `LogLevel` directly from a string using the `parse()` method.  
For example, if the log contains `"INFO"` or `"error"`, it will automatically convert it into the correct enum variant like `LogLevel::Info` or `LogLevel::Error` (or return an error if the level is invalid).

Supported log levels:

- TRACE
- DEBUG
- INFO
- WARN
- ERROR
- FATAL

Features:

- Case-insensitive parsing
- `Display` implementation (converts the enum back to a string)
- Ordering is defined as:

    Trace < Debug < Info < Warn < Error < Fatal

### 3. Log Entry Parser

Implemented the `parse_log_line()` function to convert a single log line into a structured `LogEntry`.
This function takes a log string and the source file path, then parses and validates each field.  
If the line is valid, it returns a `LogEntry`. Otherwise, it returns a descriptive `ParseError`
containing the file path, original line content, and failure reason..

A `LogEntry` contains:

- `timestamp` → the date and time of the log entry (`DateTime`)
- `level` → the severity level of the log (`LogLevel`)
- `component` → the service/module name that generated the log
- `message` → the actual log message
- `source_file` → the file path from where the log entry was read (`PathBuf`)

Example log line:
2024-01-15 10:23:45 [ERROR] storage: Failed to mount filesystem /dev/sda1

## 4. Statistics Aggregator

Created a `Statistics` module to calculate summary information from log entries.

It provides:
- total number of log entries
- entries count by level, component, and hour
- total errors (`ERROR` + `FATAL`) and error rate
- most active component and peak hour
- first and last log timestamp
- handles empty input safely

## 5. Log Analyzer Module

This module provides functionality to read, parse, and analyze log files from either a single file or an entire directory. It collects successfully parsed log entries, tracks parsing errors, and generates aggregated statistics.

- Read and parse a **single file**
- Read and parse **all `.log` files in a directory**
- Collect valid `LogEntry` records
- Collect Error `ParseError` information
- Generate aggregated `Statistics` from parsed entries
- Strong error handling using a custom `AnalyzerError` enum

---
## Data Managed by the Analyzer

A `LogAnalyzer` maintains:
- `entries: Vec<LogEntry>` — successfully parsed log entries
- `errors: Vec<ParseError>` — parsing errors encountered during analysis


## 6. Tests

All required test cases have been added

# DateTime Tests
- Parse valid datetime
- Reject invalid month (0, 13)
- Reject invalid hour (24, 25)
- Compare two datetimes

# LogLevel Tests
- Parse all valid levels
- Case insensitivity
- Ordering comparison

# Parser Tests
- Parse valid log line
- Handle extra whitespace
- Reject malformed lines with correct error

# Statistics Tests
- Calculate from sample entries
- Handle empty input
- Correct percentages

# File Processing Tests
- Process valid file
- Handle missing file
- Skip malformed lines

## 7. Reports

A separate `report.rs` file has been added to format and print
statistics in the required output styles.

- **Text Table Format** – prints summary, log level distribution,
  component distribution, peak hour, and error rate.

- **JSON Format**
  - Produces structured JSON output.

## 8. Command Line Interface (CLI)

The application supports command-line arguments for flexible usage.

### Usage
```bash
cargo run <paths> [--format text|json]

-Example: cargo run tests/fixtures/simple.log --format json