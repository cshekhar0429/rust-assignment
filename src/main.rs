mod date_time;
use date_time::DateTime;

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
    

}
