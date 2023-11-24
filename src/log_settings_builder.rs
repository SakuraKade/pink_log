use crate::pink_logger::LogLevel;
use crate::log_settings::LogSettings;

pub struct LogSettingsBuilder {
    log_settings: LogSettings
}

impl LogSettingsBuilder {
    ///
    ///
    /// # Arguments
    ///
    /// * `log_level`: Specifies what loglevel to use
    ///
    /// returns: LogSettingsBuilder
    ///
    /// # Examples
    ///
    /// ```
    /// use pink_log::{LogLevel, LogSettingsBuilder};
    ///
    /// // Minimal example
    /// let mut settings_builder = LogSettingsBuilder::new(LogLevel::Debug);
    /// let settings = settings_builder.build();
    ///
    /// ```
    pub fn new(log_level: LogLevel) -> Self {
        Self {
            log_settings: LogSettings::new(log_level)
        }
    }

    pub fn build(self) -> LogSettings {
        self.log_settings
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_settings.log_level = log_level
    }

    pub fn set_log_file(&mut self, log_file: &String) {
        self.log_settings.log_file = Some(log_file.clone());
    }

    pub fn set_write_log_to_file(&mut self, write_log_to_file: bool) {
        self.log_settings.write_log_to_file = write_log_to_file
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.log_settings.silent = silent
    }
}