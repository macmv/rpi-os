#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[inline(always)]
pub fn wfe() {
  #[cfg(target_arch = "aarch64")]
  unsafe {
    core::arch::asm!("wfe", options(nomem, nostack))
  }

  #[cfg(not(target_arch = "aarch64"))]
  unimplemented!()
}
