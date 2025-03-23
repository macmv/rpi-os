use core::fmt::Write;

pub mod driver;

/// Used by `arch` code to find the early boot core.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start_arguments")]
pub static BOOT_CORE_ID: u64 = 0;

pub unsafe fn init() {
  unsafe {
    driver::init();
  }
}

pub fn console() -> impl Write { driver::UART0 }
