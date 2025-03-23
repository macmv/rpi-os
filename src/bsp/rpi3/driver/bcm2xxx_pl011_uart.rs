use core::fmt;

use crate::{
  reg_struct,
  register::{RegRO, RegRW},
};

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

reg_struct! {
struct PL011UartRegister {
  0x00 -> dr:         RegRW<u32>,
  0x04 -> rsrecr:     RegRW<u32>,
  0x08 -> _reserved0: [u32; 4],
  0x18 -> fr:         RegRO<u32>,
  0x1c -> _reserved1: [u32; 1],
  0x20 -> ilpr:       RegRW<u32>,
  0x24 -> ibrd:       RegRW<u32>,
  0x28 -> fbrd:       RegRW<u32>,
  0x2c -> lcrh:       RegRW<u32>,
  0x30 -> cr:         RegRW<u32>,
  0x34 -> ifls:       RegRW<u32>,
  0x38 -> imsc:       RegRW<u32>,
  0x3c -> ris:        RegRW<u32>,
  0x40 -> mis:        RegRW<u32>,
  0x44 -> icr:        RegRW<u32>,
  0x48 -> dmacr:      RegRW<u32>,
}
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
  unsafe fn init(&self) { self.reg().dr.set(0x80); }
}
