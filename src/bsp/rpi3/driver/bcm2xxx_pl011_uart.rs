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
  dr:         Reg<u32>,
  rsrecr:     Reg<u32>,
  _reserved0: [u32; 4],
  fr:         Reg<u32>,
  _reserved1: [u32; 1],
  ilpr:       Reg<u32>,
  ibrd:       Reg<u32>,
  fbrd:       Reg<u32>,
  lcrh:       Reg<u32>,
  cr:         Reg<u32>,
  ifls:       Reg<u32>,
  imsc:       Reg<u32>,
  ris:        Reg<u32>,
  mis:        Reg<u32>,
  icr:        Reg<u32>,
  dmacr:      Reg<u32>,
  _reserved2: [u32; 13],
  itcr:       Reg<u32>,
  itip:       Reg<u32>,
  itop:       Reg<u32>,
  tdr:        Reg<u32>,
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

#[cfg(test)]
mod tests {
  use super::*;
  use core::mem::offset_of;

  #[test]
  fn register_offsets() {
    assert_eq!(offset_of!(PL011UartRegister, dr), 0x00);
    assert_eq!(offset_of!(PL011UartRegister, rsrecr), 0x04);
    assert_eq!(offset_of!(PL011UartRegister, fr), 0x18);
    assert_eq!(offset_of!(PL011UartRegister, ilpr), 0x20);
    assert_eq!(offset_of!(PL011UartRegister, ibrd), 0x24);
    assert_eq!(offset_of!(PL011UartRegister, fbrd), 0x28);
    assert_eq!(offset_of!(PL011UartRegister, lcrh), 0x2c);
    assert_eq!(offset_of!(PL011UartRegister, cr), 0x30);
    assert_eq!(offset_of!(PL011UartRegister, ifls), 0x34);
    assert_eq!(offset_of!(PL011UartRegister, imsc), 0x38);
    assert_eq!(offset_of!(PL011UartRegister, ris), 0x3c);
    assert_eq!(offset_of!(PL011UartRegister, mis), 0x40);
    assert_eq!(offset_of!(PL011UartRegister, icr), 0x44);
    assert_eq!(offset_of!(PL011UartRegister, dmacr), 0x48);
    assert_eq!(offset_of!(PL011UartRegister, itcr), 0x80);
    assert_eq!(offset_of!(PL011UartRegister, itip), 0x84);
    assert_eq!(offset_of!(PL011UartRegister, itop), 0x88);
    assert_eq!(offset_of!(PL011UartRegister, tdr), 0x8c);
  }
}
