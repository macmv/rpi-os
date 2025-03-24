#![feature(format_args_nl)]
#![cfg_attr(not(test), no_main)]
#![no_std]

mod arch;
mod bsp;
mod print;
mod register;

#[unsafe(no_mangle)]
fn _start_rust() -> ! {
  // SAFETY: We only call `init` once.
  unsafe {
    bsp::init();
  }

  println!("HELLO WORLD!!");

  loop {
    // let c = bsp::driver::UART0.get();
    // println!("character: {:?}", c as char);

    println!("hi");
  }
}

#[cfg(not(test))]
fn panic_prevent_reenter() {
  static mut PANIC_IN_PROGRESS: bool = false;

  unsafe {
    if !PANIC_IN_PROGRESS {
      PANIC_IN_PROGRESS = true;

      return;
    }
  }

  arch::wait_forever()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  // Protect against panic infinite loops if any of the following code panics
  // itself.
  panic_prevent_reenter();

  let (location, line, column) = match info.location() {
    Some(loc) => (loc.file(), loc.line(), loc.column()),
    _ => ("???", 0, 0),
  };

  println!("=== kernel panic ===\nat {}:{}:{}: {}", location, line, column, info.message());

  arch::wait_forever()
}
