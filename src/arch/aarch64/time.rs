use core::{num::NonZero, time::Duration};

static mut TIMER_COUNTER_FREQUENCY: NonZero<u32> = NonZero::<u32>::MIN;

mod asm {
  pub fn cntfrq_el0() -> u64 {
    let x: u64;
    unsafe { core::arch::asm!("mrs {x:x}, CNTFRQ_EL0", x = out(reg) x) }
    x
  }

  pub fn cntpct_el0() -> u64 {
    let x: u64;
    unsafe { core::arch::asm!("mrs {x:x}, CNTPCT_EL0", x = out(reg) x) }
    x
  }
}

pub unsafe fn init() {
  unsafe {
    TIMER_COUNTER_FREQUENCY =
      NonZero::new(asm::cntfrq_el0() as u32).expect("timer counter frequency is zero");
  }
}

fn timer_counter_frequency() -> NonZero<u32> { unsafe { TIMER_COUNTER_FREQUENCY } }

pub fn time_since_boot() -> Duration {
  let count = asm::cntpct_el0();

  let freq = timer_counter_frequency().get() as u64;

  let secs = count / freq;
  let subsecs = count % freq;

  Duration::new(secs, ((subsecs * 1_000_000_000) / freq) as u32)
}
