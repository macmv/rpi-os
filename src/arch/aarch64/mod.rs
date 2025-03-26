use core::arch::global_asm;

mod time;

pub use time::time_since_boot;

pub unsafe fn init() {
  unsafe {
    time::init();
  }
}

// Assembly counterpart to this file.
global_asm!(
  include_str!("boot.s"),
  CONST_CORE_ID_MASK = const 0b11
);

mod asm {
  #[inline(always)]
  pub fn wfe() { unsafe { core::arch::asm!("wfe", options(nomem, nostack)) } }

  #[inline(always)]
  pub fn nop() { unsafe { core::arch::asm!("nop", options(nomem, nostack)) } }
}

pub fn spin_for_cycles(n: usize) {
  for _ in 0..n {
    asm::nop();
  }
}

pub fn wait_forever() -> ! {
  loop {
    asm::wfe();
  }
}
