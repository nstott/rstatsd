extern crate log;

use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter, SetLoggerError};

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
	fn enabled(&self, metadata: &LogMetadata) -> bool {
		metadata.level() <= LogLevel::Info
	}

	fn log(&self, record: &LogRecord) {
		if self.enabled(record.metadata()) {
			println!("{} - {}", record.level(), record.args());
		}
	}
} 
 
impl SimpleLogger {
	pub fn init() -> Result<(), SetLoggerError> {
	    log::set_logger(|max_log_level| {
	        max_log_level.set(LogLevelFilter::Info);
	        Box::new(SimpleLogger)
	    })
	}
}