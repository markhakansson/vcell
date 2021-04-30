//! Just like [`Cell`] but with [volatile] read / write operations
//!
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html

//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
#![allow(dead_code)]
use core::cell::UnsafeCell;
#[cfg(not(feature = "klee-analysis"))]
use core::ptr;

#[cfg(feature = "klee-analysis")]
extern crate klee_rs;

#[cfg(feature = "klee-analysis")]
use klee_rs::klee_make_symbolic;

#[cfg(feature = "klee-replay")]
extern crate cortex_m_asm;

#[cfg(feature = "klee-replay")]
use cortex_m_asm::asm;

/// Just like [`Cell`] but with [volatile] read / write operations
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

#[cfg(not(feature = "klee-analysis"))]
#[cfg(not(feature = "klee-replay"))]
// NOTE implicit because of `UnsafeCell`
// unsafe impl<T> !Sync for VolatileCell<T> {}
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

#[cfg(feature = "klee-analysis")]
#[cfg(not(feature = "klee-replay"))]
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
        klee_make_symbolic(&mut symbolic_value, "vcell::get");
        symbolic_value
    }

    /// Sets the contained value
    #[inline(always)]
    pub fn set(&self, _value: T)
    where
        T: Copy,
    {
    }

    /// Returns a raw pointer to the underlying data in the cell
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        let mut symbolic_value = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        klee_make_symbolic(&mut symbolic_value, "vcell::as_ptr");
        symbolic_value
    }
}

#[cfg(not(feature = "klee-analysis"))]
#[cfg(feature = "klee-replay")]
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
        let r = unsafe { ptr::read_volatile(self.value.get()) };
        asm::bkpt_imm(5);
        r
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
        let r = self.value.get();
        asm::bkpt_imm(5);
        r
    }
}
