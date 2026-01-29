use std::path::{Path,PathBuf};
use crate::date_time::DateTime;
use crate::log_level::LogLevel;

#[derive(Debug)]
pub struct ParseError{
    pub file: PathBuf,
    pub content: String,
    pub line_number: usize,
    pub reason: String
}

#[derive(Debug)]
pub struct LogEntry {
 pub timestamp: DateTime,
 pub level: LogLevel,
 pub component: String,
 pub message: String,
 pub source_file: PathBuf,
}

fn missing_field_error(source_file: &Path, line: &str,  line_number:usize, field: &str) -> ParseError {
    ParseError {
        file: source_file.to_path_buf(),
        content: line.to_string(),
        line_number,
        reason: format!("Missing required field: {}", field),
    }
}


// line eg:- 2024-01-15 10:23:45 [ERROR] storage: Failed to mount filesystem /dev/sda1

pub fn parse_log_line(line: &str, source_file: &Path, line_number:usize) -> Result<LogEntry, ParseError> {
    let line = line.trim();

    if line.is_empty() {
        return Err(ParseError{
            file:source_file.to_path_buf(),
            content:line.to_string(),
            line_number,
            reason:"Empty line".to_string()});
    }
    let line_parts:Vec<&str> = line.split_whitespace().collect();

    // Extracting Timestamp
    let date = *line_parts.get(0)
                                .ok_or_else(|| missing_field_error(source_file, line, line_number, "date"))?;

    let time = *line_parts.get(1)
                                .ok_or_else(|| missing_field_error(source_file, line, line_number ,"time"))?;

     let dt_str = format!("{} {}", date, time);

    let timestamp: DateTime = dt_str.parse().map_err(|e| ParseError {
        file: source_file.to_path_buf(),
        content: line.to_string(),
        line_number,
        reason: format!("Invalid timestamp: {}", e),
    })?;

    // Log Level
    let log_level = *line_parts.get(2)
                                        .ok_or_else(|| missing_field_error(source_file, line, line_number,"log level"))?;

    if !log_level.starts_with('[') || !log_level.ends_with(']') {
        return Err(ParseError {
            file: source_file.to_path_buf(),
            content: line.to_string(),
            line_number,
            reason: format!("level must be in [LEVEL] format, got '{}'", log_level),
        });
    }

    let log_level = log_level.trim_start_matches('[').trim_end_matches(']');

    let level: LogLevel = log_level.parse().map_err(|e| ParseError {
        file: source_file.to_path_buf(),
        content: line.to_string(),
        line_number,
        reason: format!("Invalid log level: {}", e),
    })?;

    // Component parts 
    let component_token = *line_parts.get(3)
                                            .ok_or_else(|| missing_field_error(source_file, line, line_number, "component"))?;

    if !component_token.ends_with(':') {
        return Err(ParseError {
            file: source_file.to_path_buf(),
            content: line.to_string(),
            line_number,
            reason: format!("component must end with ':', got '{}'", component_token),
        });
    }

    let component = component_token.trim_end_matches(':').to_string();

    // Messages
    if line_parts.len() < 5 {
        return Err(missing_field_error(source_file, line, line_number, "message"));
    }

    let message = line_parts[4..].join(" ");

    if message.trim().is_empty() {
        return Err(ParseError {
            file: source_file.to_path_buf(),
            content: line.to_string(),
            line_number,
            reason: "Missing message field".to_string(),
        });
    }

    
    let log_entry:LogEntry = LogEntry{
        timestamp,
        level,
        component,
        message,
        source_file:source_file.to_path_buf()
    };

     Ok(log_entry)

}