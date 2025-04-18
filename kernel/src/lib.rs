#![no_std]

use core::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};

pub struct OnceLock<T>{
    pub once: AtomicBool,
    pub value: UnsafeCell<MaybeUninit<T>>
}
pub enum OnceLocStatus {
    Success,
    InitErr
}
impl<T> OnceLock<T> {
    pub const fn new() -> Self {
        Self {
            once: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit())
        }
    }
    
    fn is_initialized(&self) -> bool {
        self.once.load(core::sync::atomic::Ordering::SeqCst)
    }

    fn initialize<F>(&self, f: F) -> OnceLocStatus
    where
        F: FnOnce() -> T
    {
        unsafe{
            match self.value.get().as_mut(){
                Some(v) => {v.write(f());},
                None => return OnceLocStatus::InitErr
            }
            self.once.store(true, core::sync::atomic::Ordering::SeqCst);
        }
        OnceLocStatus::Success
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


    pub fn get_or_init<F, E> (&self, f: F) -> &T
    where
        F: FnOnce() -> T
    {
        match self.get() {
            Some(val) => {return val},
            None => {self.initialize(f);},
        }
        unsafe{self.get_value_unchecked()}
    }

    pub fn init<F> (&self, f: F) -> OnceLocStatus
    where
        F: FnOnce() -> T,
    {
        match self.is_initialized() {
            true => OnceLocStatus::InitErr,
            false => self.initialize(f)
        }
    }
}
unsafe impl<T> Sync for OnceLock<T>{}