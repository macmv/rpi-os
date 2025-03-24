#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(target_arch = "aarch64"))]
mod tmp {
  pub fn wfe() { unimplemented!() }
}

#[cfg(not(target_arch = "aarch64"))]
pub use tmp::*;
