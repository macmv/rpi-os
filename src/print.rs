use core::{fmt::Write, ops::Deref};

use log::Log;

use crate::bsp;

struct Logger;

pub fn init() {
  log::set_max_level(log::LevelFilter::Trace);
  log::set_logger(&Logger).unwrap();
}

impl Log for Logger {
  fn enabled(&self, _metadata: &log::Metadata) -> bool { true }

  fn log(&self, record: &log::Record) {
    if self.enabled(record.metadata()) {
      writeln!(
        bsp::console(),
        "[{} {:?}] {}",
        match record.level() {
          log::Level::Error => "E",
          log::Level::Warn => "W",
          log::Level::Info => "I",
          log::Level::Debug => "D",
          log::Level::Trace => "T",
        },
        crate::arch::time_since_boot(),
        record.args()
      )
      .unwrap();
    }
  }

  fn flush(&self) {}
}
