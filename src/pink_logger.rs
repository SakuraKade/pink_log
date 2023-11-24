use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    None
}

impl LogLevel {
    pub fn log_level_index(self) -> i32 {
        match self {
            LogLevel::Trace => 5,
            LogLevel::Debug => 4,
            LogLevel::Info => 3,
            LogLevel::Warn => 2,
            LogLevel::Error => 1,
            LogLevel::Fatal => 0,
            LogLevel::None => -1
        }
    }

    pub fn should_log(a: &Self, b: &Self) -> bool {
        if a.eq(&LogLevel::None) {
            return false
        }

        a.log_level_index() >= b.log_level_index()
    }
}

impl Into<u32> for LogLevel {
    fn into(self) -> u32 {
        self.log_level_index() as u32
    }
}

impl Into<i32> for LogLevel {
    fn into(self) -> i32 {
        self.log_level_index()
    }
}

impl Debug for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}