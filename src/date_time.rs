use std::str::FromStr;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, time) = s.split_once(' ')
                                        .ok_or_else(|| "Invalid format in date and time".to_string())?;

        println!("date={} ,time= {}",date, time);

        let (year, month, day) = {
            let mut it = date.split('-');
            match (it.next(), it.next(), it.next(), it.next()) {
                (Some(y), Some(m), Some(d), None)
                    if !y.is_empty() && !m.is_empty() && !d.is_empty() => {
                        (y, m, d)
                    }
                _ => return Err("Invalid Date Format".to_string()),
            }
        };

        let (hour, minute, second) = {
            let mut it = time.split(':');
            match (it.next(), it.next(), it.next(), it.next()) {
                (Some(h), Some(m), Some(s), None) 
                if !h.is_empty() && !m.is_empty() && !s.is_empty() => {
                    (h, m, s)
                }
                _ => return Err("Invalid time format".to_string()),
            }
        };

        let year: u16 = year.parse().map_err(|_| "error parsing year")?;
        let month: u8 = month.parse().map_err(|_| "error parsing month")?;
        let day: u8 = day.parse().map_err(|_| "error parsing day")?;

        let hour: u8 = hour.parse().map_err(|_| "error parsing hour")?;
        let minute: u8 = minute.parse().map_err(|_| "error parsing minute")?;
        let second: u8 = second.parse().map_err(|_| "error parsing second")?;

        if year < 1970 || year > 9999 {
            return Err("year Invalid".to_string());
        }
        if month < 1 || month > 12{
            return Err("month Invalid".to_string());
        }
        if day < 1 || day > 31{
            return Err("day Invalid".to_string());
        }
        if hour > 23{
            return Err("hour Invalid".to_string());
        }
        if minute > 59{
            return Err("minute Invalid".to_string());
        }
        if second > 59{
            return Err("second Invalid".to_string());
        }
        Ok(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{} {}:{}:{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl Ord for DateTime {

    fn cmp(&self, other:&Self) -> Ordering{

        (self.year, self.month, self.day, self.hour, self.minute, self.second)
            .cmp(&(other.year, other.month, other.day, other.hour, other.minute, other.second))

    }
}

impl PartialOrd for DateTime {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.cmp(other))
    }
}
