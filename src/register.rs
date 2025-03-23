use core::cell::UnsafeCell;

pub struct RegRW<T> {
  value: UnsafeCell<T>,
}
pub struct RegRO<T> {
  value: T,
}

impl<T: Clone + Copy> RegRO<T> {
  pub fn get(&self) -> T { self.value }
}

impl<T: Clone + Copy> RegRW<T> {
  pub fn get(&self) -> T { unsafe { core::ptr::read_volatile(self.value.get()) } }

  pub fn set(&self, value: T) { unsafe { core::ptr::write_volatile(self.value.get(), value) } }
}

#[macro_export]
macro_rules! reg_struct {
  (
    struct $name:ident {
      $(
        $offset:literal -> $field:ident: $reg:ty,
      )*
    }
  ) => {
    #[repr(C)]
    pub struct $name {
      $(
        pub $field: $reg,
      )*
    }

    #[cfg(test)]
    mod tests {
      use super::*;
      use core::mem::offset_of;

      #[test]
      fn register_offsets() {
        $(
          assert_eq!(offset_of!($name, $field), $offset);
        )*
      }
    }
  };
}
