use crate::register::Reg;

#[repr(C)]
pub struct PL011Uart {
  reg: &'static PL011UartRegister,
}

#[repr(C)]
struct PL011UartRegister {
  status: Reg<u8>,
}

impl PL011Uart {
  pub const unsafe fn new(base: usize) -> Self {
    unsafe { PL011Uart { reg: &*(base as *const PL011UartRegister) } }
  }

  pub fn init(&self) { self.reg.status.set(0x80); }
}
