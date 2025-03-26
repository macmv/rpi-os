use core::fmt;

use bitflags::bitflags;
use rpi_os_macros::reg_struct;

use crate::register::{RegRO, RegRW, RegWO};

pub const UART0: PL011Uart = unsafe { PL011Uart::new(0x3f20_1000) };

const UART_CLOCK: u32 = 48_000_000_u32; // 48 MHz clock

pub unsafe fn init() {
  unsafe {
    UART0.init();
  }
}

pub struct PL011Uart {
  reg: *const PL011UartRegister,
}

impl core::ops::Deref for PL011Uart {
  type Target = PL011UartRegister;

  fn deref(&self) -> &Self::Target { unsafe { &*self.reg } }
}

reg_struct! {
  struct PL011UartRegister {
    0x00 -> dr:     RegRW<u8>,
    0x04 -> rsrecr: RegRW<u8>,
    0x18 -> fr:     RegRO<u16 = Flags>,
    0x20 -> ilpr:   RegRW<u8>,
    0x24 -> ibrd:   RegRW<u16>,
    0x28 -> fbrd:   RegRW<u8>,
    0x2c -> lcrh:   RegRW<u8>,
    0x30 -> cr:     RegRW<u16 = Control>,
    0x34 -> ifls:   RegRW<u8>,
    0x38 -> imsc:   RegRW<u16>,
    0x3c -> ris:    RegRO<u16>,
    0x40 -> mis:    RegRO<u16>,
    0x44 -> icr:    RegWO<u16>,
    0x48 -> dmacr:  RegRW<u8>,
  }
}

bitflags! {
  #[derive(Clone, Copy)]
  pub struct Flags: u16 {
    const CTS  = 1 << 0;
    const DSR  = 1 << 1;
    const DCD  = 1 << 2;
    const BUSY = 1 << 3;
    const RXFE = 1 << 4;
    const TXFF = 1 << 5;
    const RXFF = 1 << 6;
    const TXFE = 1 << 7;
    const RI   = 1 << 8;
  }

  #[derive(Clone, Copy)]
  pub struct Control: u16 {
    const UARTEN = 1 << 0;
    const SIREN  = 1 << 1;
    const SIRLP  = 1 << 2;
    const LBE    = 1 << 7;
    const TXE    = 1 << 8;
    const RXE    = 1 << 9;
    const DTR    = 1 << 10;
    const RTS    = 1 << 11;
    const OUT1   = 1 << 12;
    const OUT2   = 1 << 13;
    const RTSEN  = 1 << 14;
    const CTSEN  = 1 << 15;
  }
}

impl fmt::Write for PL011Uart {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for b in s.bytes() {
      self.put(b);
    }

    Ok(())
  }
}

impl PL011Uart {
  /// SAFETY: The base address must be a valid UART address.
  pub const unsafe fn new(base: usize) -> Self {
    PL011Uart { reg: base as *const PL011UartRegister }
  }

  /// SAFETY: Must only be called once.
  unsafe fn init(&self) {
    // Disable everything.
    self.cr.set(Control::empty());

    // Clear all pending interrupts.
    self.icr.set(0xffff);

    let baud = 115200_u32;

    let ibrd = UART_CLOCK / (16 * baud);
    let fbrd = ((UART_CLOCK % (16 * baud)) * 64 + baud * 8) / (16 * baud);

    self.ibrd.set(ibrd.try_into().unwrap());
    self.fbrd.set(fbrd.try_into().unwrap());

    self.lcrh.set(0b01100000);

    // Enable UART, receive, and transmit.
    self.cr.modify(|r| r | Control::UARTEN | Control::TXE | Control::RXE);
  }

  pub fn put(&self, c: u8) {
    while self.fr.get().contains(Flags::TXFF) {}

    self.dr.set(c);
  }

  pub fn get(&self) -> u8 {
    while self.fr.get().contains(Flags::RXFE) {}

    self.dr.get()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn register_offsets() {
    // Sanity check the `reg_struct!` macro.
    assert_eq!(core::mem::offset_of!(PL011UartRegister, dr), 0x00);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, rsrecr), 0x04);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, fr), 0x18);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, ilpr), 0x20);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, ibrd), 0x24);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, fbrd), 0x28);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, lcrh), 0x2c);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, cr), 0x30);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, ifls), 0x34);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, imsc), 0x38);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, ris), 0x3c);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, mis), 0x40);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, icr), 0x44);
    assert_eq!(core::mem::offset_of!(PL011UartRegister, dmacr), 0x48);
  }
}
