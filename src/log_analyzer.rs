use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::log_entry::{LogEntry, ParseError};
use crate::statistics_aggregator::Statistics;
use crate::log_entry::parse_log_line;

#[derive(Debug)]
pub enum AnalyzerError {
    IoError { path: PathBuf, source: io::Error },
    NoFilesFound { path: PathBuf },
    InvalidPath { path: PathBuf, reason: String },
}

pub struct LogAnalyzer {
    entries: Vec<LogEntry>,
    errors: Vec<ParseError>,
}

impl LogAnalyzer {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    // Read one log file.
    pub fn process_file(&mut self, path: &Path) -> Result<usize, AnalyzerError> {
        if !path.exists() {
            return Err(AnalyzerError::InvalidPath {
                path: path.to_path_buf(),
                reason: "Path does not exist".to_string(),
            });
        }

        if !path.is_file() {
            return Err(AnalyzerError::InvalidPath {
                path: path.to_path_buf(),
                reason: "Expected a file path".to_string(),
            });
        }

        let mut file = File::open(path).map_err(|e| AnalyzerError::IoError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| AnalyzerError::IoError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let mut success_count = 0usize;

        for (i, line) in contents.lines().enumerate() {
            match parse_log_line(line, path, i+1) {
                Ok(entry) => {
                    self.entries.push(entry);
                    success_count += 1;
                }
                Err(err) => {
                    self.errors.push(err);
                }
            }
        }

        Ok(success_count)
    }

    // Read all log files in a directory 
    pub fn process_directory(&mut self, path: &Path) -> Result<usize, AnalyzerError> {
        if !path.exists() {
            return Err(AnalyzerError::InvalidPath {
                path: path.to_path_buf(),
                reason: "Path does not exist".to_string(),
            });
        }

        if !path.is_dir() {
            return Err(AnalyzerError::InvalidPath {
                path: path.to_path_buf(),
                reason: "Expected a directory path".to_string(),
            });
        }

        let dir_iter = fs::read_dir(path).map_err(|e| AnalyzerError::IoError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let mut total_success = 0usize;
        let mut found_log_file = false;

        for entry in dir_iter {
            let entry = entry.map_err(|e| AnalyzerError::IoError {
                path: path.to_path_buf(),
                source: e,
            })?;

            let file_path = entry.path();

            let is_log_file = file_path.is_file()
                && file_path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("log"))
                    .unwrap_or(false);

            if is_log_file {
                found_log_file = true;
                total_success += self.process_file(&file_path)?;
            }
        }

        if !found_log_file {
            return Err(AnalyzerError::NoFilesFound {
                path: path.to_path_buf(),
            });
        }

        Ok(total_success)
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn parse_errors(&self) -> &[ParseError] {
        &self.errors
    }

    pub fn statistics(&self) -> Statistics {
        Statistics::from_entries(&self.entries)
    }
}
