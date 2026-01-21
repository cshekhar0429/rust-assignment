use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
 Trace,
 Debug,
 Info,
 Warn,
 Error,
 Fatal,
}

impl FromStr for LogLevel{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
       match s.to_lowercase().as_str() {
        "trace" => Ok(LogLevel::Trace),
        "debug" => Ok(LogLevel::Debug),
        "info" => Ok(LogLevel::Info),
        "warn" => Ok(LogLevel::Warn),
        "error" => Ok(LogLevel::Error),
        "fatal" => Ok(LogLevel::Fatal),
        _ => Err("Log Level is invalid".to_string())
       }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
       let s =  match self {
            LogLevel::Trace => "Trace",
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warn => "Warn",
            LogLevel::Error => "Error",
            LogLevel::Fatal => "Fatal",
        };
        write!(f,"{}",s)
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> Ordering{
        (*self as u8).cmp(&(*other as u8))
    }
}
