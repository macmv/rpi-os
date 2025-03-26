#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(target_arch = "aarch64"))]
mod tmp {
  #![allow(unused_variables)]

  use core::time::Duration;

  pub unsafe fn init() { unimplemented!() }
  pub fn wait_forever() -> ! { unimplemented!() }
  pub fn spin_for_cycles(n: usize) { unimplemented!() }
  pub fn time_since_boot() -> Duration { unimplemented!() }
}

#[cfg(not(target_arch = "aarch64"))]
pub use tmp::*;
