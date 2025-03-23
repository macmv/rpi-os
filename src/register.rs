use core::cell::UnsafeCell;

pub struct Reg<T> {
  value: UnsafeCell<T>,
}

impl<T: Clone + Copy> Reg<T> {
  pub fn get(&self) -> T { unsafe { core::ptr::read_volatile(self.value.get()) } }

  pub fn set(&self, value: T) { unsafe { core::ptr::write_volatile(self.value.get(), value) } }
}
