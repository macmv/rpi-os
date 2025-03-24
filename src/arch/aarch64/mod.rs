use core::arch::global_asm;

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

pub fn wait_forever() -> ! {
  loop {
    asm::wfe();
  }
}
