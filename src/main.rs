#![cfg_attr(not(test), no_main)]
#![no_std]

mod arch;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { unimplemented!() }
