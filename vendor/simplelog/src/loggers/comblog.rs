// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module providing the CombinedLogger Implementation

use log::{LogLevelFilter, LogMetadata, LogRecord, SetLoggerError, set_logger, Log};
use ::{SharedLogger, Config};

/// The CombinedLogger struct. Provides a Logger implementation that proxies multiple Loggers as one.
///
/// The purpose is to allow multiple Loggers to be set globally
pub struct CombinedLogger {
    level: LogLevelFilter,
    logger: Vec<Box<SharedLogger>>,
}

impl CombinedLogger {

    /// init function. Globally initializes the CombinedLogger as the one and only used log facility.
    ///
    /// Takes all used Loggers as a Vector argument. None of those Loggers should already be set globally.
    ///
    /// All loggers need to implement `log::Log` and `logger::SharedLogger` and need to provide a way to be
    /// initialized without calling `set_logger`. All loggers of this library provide a `new(..)`` method
    /// for that purpose.
    /// Fails if another logger is already set globally.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # use std::fs::File;
    /// # fn main() {
    /// let _ = CombinedLogger::init(
    ///             vec![
    ///                 TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
    ///                 WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("my_rust_bin.log").unwrap())
    ///             ]
    ///         );
    /// # }
    /// ```
    pub fn init(logger: Vec<Box<SharedLogger>>) -> Result<(), SetLoggerError> {
        set_logger(|max_log_level| {
            let result = CombinedLogger::new(logger);
            max_log_level.set(result.level());
            result
        })
    }

    /// allows to create a new logger, that can be independently used, no matter whats globally set.
    ///
    /// no macros are provided for this case and you probably
    /// dont want to use this function, but `init()`, if you dont want to build a `CombinedLogger`.
    ///
    /// Takes all used loggers as a Vector argument. The log level is automatically determined by the
    /// lowest log level used by the given loggers.
    ///
    /// All loggers need to implement log::Log.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # use std::fs::File;
    /// # fn main() {
    /// let combined_logger = CombinedLogger::new(
    ///             vec![
    ///                 TermLogger::new(LogLevelFilter::Debug, Config::default()).unwrap(),
    ///                 WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("my_rust_bin.log").unwrap())
    ///             ]
    ///         );
    /// # }
    /// ```
    pub fn new(logger: Vec<Box<SharedLogger>>) -> Box<CombinedLogger> {
        let mut log_level = LogLevelFilter::Off;
        for log in &logger {
            if log_level < log.level() {
                log_level = log.level();
            }
        }

        Box::new(CombinedLogger { level: log_level, logger: logger })
    }

}

impl Log for CombinedLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            for log in &self.logger {
                log.log(record);
            }
        }
    }
}

impl SharedLogger for CombinedLogger {

    fn level(&self) -> LogLevelFilter {
        self.level
    }

    fn config(&self) -> Option<&Config>
    {
        None
    }

    fn as_log(self: Box<Self>) -> Box<Log> {
        Box::new(*self)
    }

}
