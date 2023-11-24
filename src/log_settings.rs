use crate::pink_logger::LogLevel;

pub struct LogSettings {
    pub log_level: LogLevel, // Required
    pub log_file: Option<String>, // Default: none, Autogenerate if none
    pub write_log_to_file: bool, // Default: true, Tells the logger to write to log file
    pub silent: bool // Default: false, Suppresses console output
}

impl LogSettings {
    pub(crate) fn new(log_level: LogLevel) -> Self {
        Self {
            log_level,
            log_file: None,
            write_log_to_file: true,
            silent: false
        }
    }
}

#[test]
fn test_log_settings_new() {
    let log_settings = LogSettings::new(LogLevel::Debug);

    // Check the default
    assert_eq!(log_settings.log_level, LogLevel::Debug);
    assert_eq!(log_settings.log_file, None);
    assert_eq!(log_settings.write_log_to_file, true);
    assert_eq!(log_settings.silent, false);
}