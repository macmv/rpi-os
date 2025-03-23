#[cfg(any(feature = "bsp_rpi3", test))]
mod rpi3;

#[cfg(feature = "bsp_rpi3")]
pub use rpi3::*;

#[cfg(not(feature = "bsp_rpi3"))]
mod blank {
  use core::fmt::{self, Write};

  struct BlankOutput;

  impl fmt::Write for BlankOutput {
    fn write_str(&mut self, _: &str) -> fmt::Result { Ok(()) }
  }

  pub fn console() -> impl Write { BlankOutput }
  pub fn init() {}
}

#[cfg(not(feature = "bsp_rpi3"))]
pub use blank::*;
