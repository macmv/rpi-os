mod bcm2xxx_pl011_uart;

pub use bcm2xxx_pl011_uart::PL011Uart;

const UART0: PL011Uart = unsafe { PL011Uart::new(0x3F20_1000) };

pub unsafe fn init() { UART0.init(); }
