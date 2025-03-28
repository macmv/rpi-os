use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct RegRO<T> {
  value: T,
}
#[repr(transparent)]
pub struct RegWO<T> {
  value: UnsafeCell<T>,
}
#[repr(transparent)]
pub struct RegRW<T> {
  value: UnsafeCell<T>,
}

impl<T: Clone + Copy> RegRO<T> {
  pub fn get(&self) -> T { unsafe { core::ptr::read_volatile(&self.value) } }
}

impl<T: Clone + Copy> RegWO<T> {
  pub fn set(&self, value: T) { unsafe { core::ptr::write_volatile(self.value.get(), value) } }
}

impl<T: Clone + Copy> RegRW<T> {
  pub fn get(&self) -> T { unsafe { core::ptr::read_volatile(self.value.get()) } }

  pub fn set(&self, value: T) { unsafe { core::ptr::write_volatile(self.value.get(), value) } }

  pub fn modify(&self, f: impl FnOnce(T) -> T) { self.set(f(self.get())); }
}
