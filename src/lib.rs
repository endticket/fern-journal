extern crate core;
extern crate log;
extern crate fern;
extern crate systemd;

use core::fmt;

pub struct JournalLogger {}

impl JournalLogger {
    fn new() -> JournalLogger {
        JournalLogger {}
    }
}

impl fern::FernLog for JournalLogger {
    fn log_args(&self, _message: &fmt::Arguments, record: &log::LogRecord) {
        systemd::journal::log_record(record);
        // FIXME: the formatted output is not used, is it a problem?
        // currently we do not want the timestamp and other spam in the journal, but later on this feature could be required
    }
}

pub fn get_logger() -> Box<fern::FernLog> {
    Box::new(JournalLogger::new())
}
