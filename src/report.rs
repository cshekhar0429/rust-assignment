use crate::statistics_aggregator::Statistics;

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
