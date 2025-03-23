use core::fmt;

use crate::register::Reg;

#[repr(C)]
pub struct PL011Uart {
  reg: *const PL011UartRegister,
}

pub const UART0: PL011Uart = unsafe { PL011Uart::new(0x3F20_1000) };

pub unsafe fn init() {
  unsafe {
    UART0.init();
  }
}

#[repr(C)]
struct PL011UartRegister {
  status: Reg<u8>,
}

impl fmt::Write for PL011Uart {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for b in s.bytes() {
      unsafe {
        core::ptr::write_volatile(self.reg as *mut u8, b);
      }
    }

    Ok(())
  }
}

impl PL011Uart {
  /// SAFETY: The base address must be a valid UART address.
  pub const unsafe fn new(base: usize) -> Self {
    PL011Uart { reg: base as *const PL011UartRegister }
  }

  fn reg(&self) -> &PL011UartRegister { unsafe { &*self.reg } }

  /// SAFETY: Must only be called once.
  unsafe fn init(&self) { self.reg().status.set(0x80); }
}
