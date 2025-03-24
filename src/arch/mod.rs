#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(target_arch = "aarch64"))]
mod tmp {
  #![allow(unused_variables)]

  pub fn wait_forever() -> ! { unimplemented!() }
  pub fn spin_for_cycles(n: usize) { unimplemented!() }
}

#[cfg(not(target_arch = "aarch64"))]
pub use tmp::*;
