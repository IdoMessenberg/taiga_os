#![no_std]

use core::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};

pub struct OnceLock<T>{
    pub once: AtomicBool,
    pub value: UnsafeCell<MaybeUninit<T>>
}
impl<T> OnceLock<T> {
    pub const fn new() -> Self {
        Self {
            once: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit())
        }
    }
    
    fn is_initialized(&self) -> bool {
        self.once.load(core::sync::atomic::Ordering::Relaxed)
    }

    fn initialize<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<T, E>
    {
        match f() {
            Ok(val) => {
                unsafe{
                    self.value.get().as_mut().unwrap().write(val);
                    self.once.store(true, core::sync::atomic::Ordering::Release);
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    pub unsafe fn get_value_unchecked(&self) -> &T {
        unsafe{(&*self.value.get()).assume_init_ref()}
    }

    unsafe fn get_value_unchecked_mut(&self) -> &mut T {
        unsafe{(&mut *self.value.get()).assume_init_mut()}
    }

    pub fn get(&self) -> Option<&T> {
        match self.is_initialized() {
            true => unsafe{Some(self.get_value_unchecked())},
            false => None
        }
    }

    pub fn get_mut (&self) -> Option<&mut T> {
        match self.is_initialized() {
            true => unsafe{Some(self.get_value_unchecked_mut())},
            false => None
        }
    }


    pub fn get_or_try_init<F, E> (&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>
    {
        match self.get() {
            Some(val) => {return Ok(val)},
            None => {self.initialize(f)?;},
        }
        Ok(unsafe{self.get_value_unchecked()})
    }

    pub fn get_or_init<F> (&self, f: F) -> &T
    where
        F: FnOnce() -> T
    {
        match self.get_or_try_init(|| Ok::<T, ()>(f())) {
            Ok(val) => val,
            Err(_handler) => unreachable!(),
        }
    }

    pub fn init<F, E> (&self, f: F, error_code: E) -> Result<(), E>
    where
        F: FnOnce() -> Result<T,E>,
    {
        match self.is_initialized() {
            true => Err(error_code),
            false => self.initialize(f)
        }
    }
}
unsafe impl<T> Sync for OnceLock<T>{}