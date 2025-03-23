#![feature(format_args_nl)]
#![cfg_attr(not(test), no_main)]
#![no_std]

mod arch;
mod bsp;
mod print;

#[unsafe(no_mangle)]
fn _start_rust() -> ! {
  println!("HELLO WORLD!!");

  unimplemented!()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {
    arch::wfe();
  }
}
