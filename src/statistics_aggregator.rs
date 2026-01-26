use crate::DateTime;
use crate::LogLevel;
use crate::log_entry::LogEntry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Statistics {
    pub total_entries: usize,
    pub entries_by_level: HashMap<LogLevel, usize>,
    pub entries_by_component: HashMap<String, usize>,
    pub entries_by_hour: HashMap<u8, usize>,
    pub error_count: usize, // ERROR + FATAL
    pub error_rate: f64,    // error_count / total_entries
    pub most_active_component: Option<String>,
    pub peak_hour: Option<u8>,
    pub first_entry: Option<DateTime>,
    pub last_entry: Option<DateTime>,
}

impl Statistics {

    pub fn from_entries(entries: &[LogEntry]) -> Self {
        let total_entries = entries.len();
        let mut entries_by_level: HashMap<LogLevel, usize> = HashMap::new();
        let mut entries_by_component: HashMap<String, usize> = HashMap::new();
        let mut entries_by_hour: HashMap<u8, usize> = HashMap::new();
        let mut error_count: usize = 0;
        let mut first_entry: Option<DateTime> = None;
        let mut last_entry: Option<DateTime> = None;

        for entry in entries {
            // Entries by Level Count
            *entries_by_level.entry(entry.level).or_insert(0) += 1;

            if let Some(v) = entries_by_component.get_mut(&entry.component) {
                *v += 1;
            }
            else {
                entries_by_component.insert(entry.component.clone(), 1);
            }

            // Entries by Hour Count
            let hour = entry.timestamp.hour;
            *entries_by_hour.entry(hour).or_insert(0) += 1;

            // Error Count
            if entry.level == LogLevel::Error || entry.level == LogLevel::Fatal {
                error_count += 1;
            }

            first_entry = match first_entry {
                None => Some(entry.timestamp.clone()),
                Some(curr) => {
                    if entry.timestamp < curr {
                        Some(entry.timestamp.clone())
                    }
                    else{
                        Some(curr)
                    }
                }
            };

            last_entry = match last_entry {
                None => Some(entry.timestamp.clone()),
                Some(curr) => {
                    if entry.timestamp > curr {
                        Some(entry.timestamp.clone())
                    } else {
                        Some(curr)
                    }
                }
            };

        }

        // most active component
        let mut most_active_component: Option<String> = None;
        let mut max_count: usize = 0;

        for (component, count) in &entries_by_component {
            if most_active_component.is_none() || *count > max_count {
                max_count = *count;
                most_active_component = Some(component.clone());
            }
        }
    
        // peak hour
        let mut peak_hour: Option<u8> = None;
        let mut max_count_peak_hour: usize = 0;

        for (hour, count) in &entries_by_hour {
            if peak_hour.is_none() || *count > max_count_peak_hour {
                max_count_peak_hour = *count;
                peak_hour = Some(*hour);
            }
        }
        // error rate
        let mut error_rate = 0.0;

        if total_entries > 0 {
            error_rate = error_count as f64 / total_entries as f64;
        }

        Statistics {
            total_entries,
            entries_by_level,
            entries_by_component,
            entries_by_hour,
            error_count,
            error_rate,
            most_active_component,
            peak_hour,
            first_entry,
            last_entry,
        }
    }
}
