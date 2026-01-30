use crate::statistics_aggregator::Statistics;
use std::collections::HashMap;
use serde::Serialize;


pub fn print_report(stats: &Statistics, parse_error_count: usize) {
    println!("================== LOG ANALYSIS REPORT ==================");

    if let (Some(first), Some(last)) = (&stats.first_entry, &stats.last_entry) {
        println!("Period: {} to {}", first, last);
    }

    println!("SUMMARY");
    println!("-------");
    println!("Total Entries: {}", stats.total_entries);
    println!("Error Rate: {:.2}%", stats.error_rate * 100.0);

    if let Some(hour) = stats.peak_hour {
        let count = stats.entries_by_hour.get(&hour).unwrap_or(&0);
        println!("Peak Hour: {:02}:00 ({} entries)", hour, count);
    }

    if let Some(comp) = &stats.most_active_component {
        let count = stats.entries_by_component.get(comp).unwrap_or(&0);
        println!("Most Active: {} ({} entries)", comp, count);
    }

    println!("BY LOG LEVEL");
    println!("------------");

    for (level, count) in &stats.entries_by_level {
        let percent = (*count as f64 / stats.total_entries as f64) * 100.0;
        println!("{:<6} {} ({:.1}%)", level, count, percent);
    }

    println!("BY COMPONENT");
    println!("------------");

    for (comp, count) in &stats.entries_by_component {
        let percent = (*count as f64 / stats.total_entries as f64) * 100.0;
        println!("{:<10} {} ({:.1}%)", comp, count, percent);
    }

    println!("Parse Errors: {} lines skipped", parse_error_count);
    println!("=========================================================");
}


// json report

#[derive(Serialize)]
struct PeriodJson {
    start: String,
    end: String,
}

#[derive(Serialize)]
struct ReportJson {
    total_entries: usize,
    error_rate: f64,
    peak_hour: Option<u8>,
    most_active_component: Option<String>,
    entries_by_level: HashMap<String, usize>,
    entries_by_component: HashMap<String, usize>,
    period: Option<PeriodJson>,
    parse_errors: usize,
}

pub fn print_report_json(stats: &Statistics, parse_error_count: usize) {

    // Convert LogLevel -> String so we don't touch enum
    let entries_by_level_json: HashMap<String, usize> =
        stats.entries_by_level
            .iter()
            .map(|(lvl, count)| (lvl.to_string(), *count))
            .collect();

    // Period
    let period_json = if let (Some(first), Some(last)) =
        (&stats.first_entry, &stats.last_entry)
    {
        Some(PeriodJson {
            start: first.to_string(),
            end: last.to_string(),
        })
    } else {
        None
    };

    // Build final ordered struct
    let report = ReportJson {
        total_entries: stats.total_entries,
        error_rate: stats.error_rate,
        peak_hour: stats.peak_hour,
        most_active_component: stats.most_active_component.clone(),
        entries_by_level: entries_by_level_json,
        entries_by_component: stats.entries_by_component.clone(),
        period: period_json,
        parse_errors: parse_error_count,
    };

    // Print JSON
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}

