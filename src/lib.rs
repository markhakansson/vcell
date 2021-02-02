//! Just like [`Cell`] but with [volatile] read / write operations
//!
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::cell::UnsafeCell;
#[cfg(not(feature = "klee-analysis"))]
use core::ptr;

#[cfg(feature = "klee-analysis")]
#[macro_use]
extern crate klee_sys;

/// Just like [`Cell`] but with [volatile] read / write operations
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

#[cfg(not(feature = "klee-analysis"))]
impl<T> VolatileCell<T> {
    /// Creates a new `VolatileCell` containing the given value
    pub const fn new(value: T) -> Self {
        VolatileCell {
            value: UnsafeCell::new(value),
        }
    }

    /// Returns a copy of the contained value
    #[inline(always)]
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { ptr::read_volatile(self.value.get()) }
    }

    /// Sets the contained value
    #[inline(always)]
    pub fn set(&self, value: T)
    where
        T: Copy,
    {
        unsafe { ptr::write_volatile(self.value.get(), value) }
    }

    /// Returns a raw pointer to the underlying data in the cell
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

// NOTE implicit because of `UnsafeCell`
// unsafe impl<T> !Sync for VolatileCell<T> {}

#[cfg(feature = "klee-analysis")]
impl<T> VolatileCell<T> {
    /// Creates a new `VolatileCell` containing the given value
    pub const fn new(value: T) -> Self {
        VolatileCell {
            value: UnsafeCell::new(value),
        }
    }

    /// Returns a copy of the contained value
    #[inline(always)]
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        let mut symbolic_value = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        klee_make_symbolic!(&mut symbolic_value, "vcell");
        symbolic_value
    }

    /// Sets the contained value
    #[inline(always)]
    pub fn set(&self, value: T)
    where
        T: Copy,
    {
    }

    /// Returns a raw pointer to the underlying data in the cell
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}
