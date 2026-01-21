use std::path::Path;
mod date_time;
use date_time::DateTime;

mod log_level;
use log_level::LogLevel;

mod log_entry;

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


}
