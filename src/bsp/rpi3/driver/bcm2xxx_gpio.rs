use rpi_os_macros::reg_struct;

use crate::register::{RegRW, RegWO};

pub const GPIO: Gpio = unsafe { Gpio::new(0x3F20_0000) };

pub unsafe fn init() {
  unsafe {
    GPIO.init();
  }
}

pub struct Gpio {
  reg: *const GpioRegister,
}

reg_struct! {
  struct GpioRegister {
    0x00 -> fsel0:   RegRW<u32>,
    0x04 -> fsel1:   RegRW<u32>,
    0x08 -> fsel2:   RegRW<u32>,
    0x0c -> fsel3:   RegRW<u32>,
    0x10 -> fsel4:   RegRW<u32>,
    0x14 -> fsel5:   RegRW<u32>,
    0x1c -> set0:    RegWO<u32>,
    0x20 -> set1:    RegWO<u32>,
    0x28 -> clr0:    RegWO<u32>,
    0x2c -> clr1:    RegWO<u32>,
    0x34 -> lev0:    RegWO<u32>,
    0x38 -> lev1:    RegWO<u32>,
    0x40 -> eds0:    RegRW<u32>,
    0x44 -> eds1:    RegRW<u32>,
    0x4c -> ren0:    RegRW<u32>,
    0x50 -> ren1:    RegRW<u32>,
    0x58 -> fen0:    RegRW<u32>,
    0x5c -> fen1:    RegRW<u32>,
    0x64 -> hen0:    RegRW<u32>,
    0x68 -> hen1:    RegRW<u32>,
    0x70 -> len0:    RegRW<u32>,
    0x74 -> len1:    RegRW<u32>,
    0x7c -> aren0:   RegRW<u32>,
    0x80 -> aren1:   RegRW<u32>,
    0x88 -> afen0:   RegRW<u32>,
    0x8c -> afen1:   RegRW<u32>,
    0x94 -> pud:     RegRW<u32>,
    0x98 -> pudclk0: RegRW<u32>,
    0x9c -> pudclk1: RegRW<u32>,
  }
}

impl Gpio {
  pub const fn new(base_addr: usize) -> Self { Self { reg: base_addr as *const GpioRegister } }

  fn reg(&self) -> &GpioRegister { unsafe { &*self.reg } }

  unsafe fn init(&self) {
    // TODO
  }
}
