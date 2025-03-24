use core::arch::global_asm;

// Assembly counterpart to this file.
global_asm!(
  include_str!("boot.s"),
  CONST_CORE_ID_MASK = const 0b11
);

#[inline(always)]
pub fn wfe() { unsafe { core::arch::asm!("wfe", options(nomem, nostack)) } }
