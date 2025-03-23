mod bcm2xxx_pl011_uart;

pub use bcm2xxx_pl011_uart::PL011Uart;

pub fn init() {
  unsafe {
    PL011Uart::new(0x3F20_1000).init();
  }
}
