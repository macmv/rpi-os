use crate::register::Reg;

#[repr(C)]
pub struct PL011Uart {
  reg: *const PL011UartRegister,
}

#[repr(C)]
struct PL011UartRegister {
  status: Reg<u8>,
}

impl PL011Uart {
  /// SAFETY: The base address must be a valid UART address.
  pub const unsafe fn new(base: usize) -> Self {
    PL011Uart { reg: base as *const PL011UartRegister }
  }

  fn reg(&self) -> &PL011UartRegister { unsafe { &*self.reg } }

  pub fn init(&self) { self.reg().status.set(0x80); }
}
