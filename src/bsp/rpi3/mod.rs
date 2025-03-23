/// Used by `arch` code to find the early boot core.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start_arguments")]
pub static BOOT_CORE_ID: u64 = 0;

const UART0_BASE: u32 = 0x3F20_1000;

use core::fmt::{self, Write};

struct QemuOutput;

impl fmt::Write for QemuOutput {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for b in s.bytes() {
      unsafe {
        core::ptr::write_volatile(UART0_BASE as *mut u8, b);
      }
    }

    Ok(())
  }
}

pub fn console() -> impl Write { QemuOutput }
