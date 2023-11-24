use std::error::Error;
use uuid::Uuid;
use std::fs;
use crate::logger::Logger;
use crate::{LogLevel, LogSettings, LogSettingsBuilder};

pub struct PinkLogger {
    log_settings: LogSettings,
    log_path: String
}

impl PinkLogger {

    ///
    ///
    /// # Arguments
    ///
    /// * `log_settings`:
    ///
    /// returns: PinkLogger
    ///
    /// # Examples
    ///
    /// ```
    /// use pink_log::{PinkLogger, LogSettings, LogLevel, LogSettingsBuilder, Logger};
    ///
    /// let mut settings = LogSettingsBuilder::new(LogLevel::Trace);
    /// settings.set_write_log_to_file(false);
    /// let settings = settings.build();
    ///
    /// let log = PinkLogger::new(settings);
    ///
    /// log.trace(None);
    /// log.trace(Some("This is a trace point"));
    /// log.debug("This is debug message");
    /// log.info("This is info");
    /// log.warn("This is a warning");
    /// log.error(Box::new(std::error::Error::new("This is an error")));
    /// log.fatal(Box::new(std::error::Error::new("This is an fatal error")));
    /// ```
    pub fn new(log_settings: LogSettings) -> Self {
        let log_path = match &log_settings.log_file {
            None => {
                let uuid = Self::generate_uuid();
                format!("pink_log/{}.txt", uuid.to_string())
            }
            Some(some) => {
                some.clone()
            }
        };

        Self {
            log_settings,
            log_path,
        }
    }

    ///
    ///
    /// returns: PinkLogger
    ///
    /// # Examples
    ///
    /// ```
    /// use pink_log::{Logger, PinkLogger};
    ///
    /// let logger = PinkLogger::default();
    /// logger.trace(None);
    /// logger.trace(Some("This is a trace point"));
    /// logger.debug("This is debug message");
    /// logger.info("This is info");
    /// logger.warn("This is a warning");
    /// logger.error(Box::new(std::error::Error::new("This is an error")));
    /// logger.fatal(Box::new(std::error::Error::new("This is an fatal error")));
    /// ```
    pub fn default() -> Self {
        let builder = LogSettingsBuilder::new(LogLevel::Info);
        let log_settings = builder.build();

        let uuid = Self::generate_uuid();
        let log_path = format!("pink_log/{}.txt", uuid.to_string());

        Self {
            log_settings,
            log_path
        }
    }

    fn generate_uuid() -> Uuid {
        Uuid::new_v4()
    }

    fn write(&self, value: String) {
        if self.log_settings.write_log_to_file {
            self.write_to_log_file(&value);
        }

        if !self.log_settings.silent {
            println!("{}", value);
        }
    }

    fn write_to_log_file(&self, value: &String) {
        if let Err(error) = fs::write(&self.log_path, value) {
            println!("[pink-log] [WARN] Error while trying to write to file:\n {}\n{:?}", &self.log_path, error)
        }
    }
}

impl Logger for PinkLogger {
    fn trace(&self, msg: Option<&str>) {
        // Guard pattern: Check if this can be logged
        if !LogLevel::should_log(&self.log_settings.log_level, &LogLevel::Trace) {
            return;
        }

        let backtrace = std::backtrace::Backtrace::capture();
        let log = match msg {
            None => format!("[TRACE] {}", backtrace),
            Some(msg) => format!("[TRACE] {} {}", msg, backtrace)
        };

        self.write(log);
    }

    fn debug(&self, msg: &str) {
        // Guard pattern: Check if this can be logged
        if !LogLevel::should_log(&self.log_settings.log_level, &LogLevel::Debug) {
            return;
        }

        let log = format!("[DEBUG] {}", msg);

        self.write(log);
    }

    fn info(&self, msg: &str) {
        // Guard pattern: Check if this can be logged
        if !LogLevel::should_log(&self.log_settings.log_level, &LogLevel::Info) {
            return;
        }

        let log = format!("[INFO] {}", msg);

        self.write(log);
    }

    fn warn(&self, msg: &str) {
        // Guard pattern: Check if this can be logged
        if !LogLevel::should_log(&self.log_settings.log_level, &LogLevel::Warn) {
            return;
        }

        let log = format!("[WARN] {}", msg);

        self.write(log);
    }

    fn error(&self, err: Box<dyn Error>) {
        // Guard pattern: Check if this can be logged
        if !LogLevel::should_log(&self.log_settings.log_level, &LogLevel::Error) {
            return;
        }

        let log = format!("[ERROR] {}", err);

        self.write(log);
    }

    fn fatal(&self, err: Box<dyn Error>) {
        // Guard pattern: Check if this can be logged
        if !LogLevel::should_log(&self.log_settings.log_level, &LogLevel::Fatal) {
            return;
        }

        let log = format!("[FATAl] {}", err);

        self.write(log);
    }
}