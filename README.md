# pink_log

A simple lightweight logger written in Rust (WIP)

WARNING: This is an early version and is not ready for beta let alone production.

Crate: https://crates.io/crates/pink_log



### Examples

##### Toml

```rust
pink_log = "0.1.0"
```

##### Using Builder

```rust
use pink_log::{PinkLogger, LogSettings, LogLevel, LogSettingsBuilder, Logger};

let mut settings = LogSettingsBuilder::new(LogLevel::Trace);
settings.set_write_log_to_file(false);
let settings = settings.build();

let log = PinkLogger::new(settings);

log.trace(None);
log.trace(Some("This is a trace point"));
log.debug("This is debug message");
log.info("This is info");
log.warn("This is a warning");
log.error(Box::new(std::error::Error::new("This is an error")));
log.fatal(Box::new(std::error::Error::new("This is an fatal error")));
```

##### Using Default

```rust
use pink_log::{Logger, PinkLogger};

let logger = PinkLogger::default();
logger.trace(None);
logger.trace(Some("This is a trace point"));
logger.debug("This is debug message");
logger.info("This is info");
logger.warn("This is a warning");
logger.error(Box::new(std::error::Error::new("This is an error")));
logger.fatal(Box::new(std::error::Error::new("This is an fatal error")));
```
