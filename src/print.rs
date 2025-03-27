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
        "[{} {}] {}",
        match record.level() {
          log::Level::Error => "E",
          log::Level::Warn => "W",
          log::Level::Info => "I",
          log::Level::Debug => "D",
          log::Level::Trace => "T",
        },
        PrettyTime(crate::arch::time_since_boot()),
        record.args()
      )
      .unwrap();
    }
  }

  fn flush(&self) {}
}

struct PrettyTime(core::time::Duration);

impl Deref for PrettyTime {
  type Target = core::time::Duration;

  fn deref(&self) -> &Self::Target { &self.0 }
}

impl core::fmt::Display for PrettyTime {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    // Each one of these is 12 characters long.
    if self.as_secs() > 0 {
      write!(f, "{:>3}.{:06}s ", self.as_secs(), self.subsec_micros())
    } else if self.as_millis() > 0 {
      let int = self.as_millis();
      let frac = self.subsec_nanos() % 1_000_000;
      write!(f, "{int:>3}.{frac:06}ms")
    } else if self.as_micros() > 0 {
      let int = self.as_micros();
      let frac = self.subsec_nanos() % 1_000;
      write!(f, "{int:>6}.{frac:03}µs")
    } else {
      write!(f, "{:>10}ns", self.as_nanos())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use core::time::Duration;

  use std::prelude::rust_2024::*;

  fn p(dur: Duration) -> String { std::format!("{}", PrettyTime(dur)) }

  #[test]
  fn pretty_time() {
    assert_eq!(p(Duration::from_nanos(1)), "         1ns");
    assert_eq!(p(Duration::from_nanos(10)), "        10ns");
    assert_eq!(p(Duration::from_nanos(100)), "       100ns");
    assert_eq!(p(Duration::from_micros(1)), "     1.000µs");
    assert_eq!(p(Duration::from_micros(10)), "    10.000µs");
    assert_eq!(p(Duration::from_micros(100)), "   100.000µs");
    assert_eq!(p(Duration::from_millis(1)), "  1.000000ms");
    assert_eq!(p(Duration::from_millis(10)), " 10.000000ms");
    assert_eq!(p(Duration::from_millis(100)), "100.000000ms");
    assert_eq!(p(Duration::from_secs(1)), "  1.000000s ");
    assert_eq!(p(Duration::from_secs(10)), " 10.000000s ");
    assert_eq!(p(Duration::from_secs(100)), "100.000000s ");
  }
}
