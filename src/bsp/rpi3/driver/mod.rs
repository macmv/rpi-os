mod bcm2xxx_gpio;
mod bcm2xxx_pl011_uart;

pub use bcm2xxx_pl011_uart::UART0;

pub unsafe fn init() {
  unsafe {
    bcm2xxx_pl011_uart::init();
    bcm2xxx_gpio::init();
  }
}
