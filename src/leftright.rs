use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

unsafe impl<T: Clone> Sync for LeftRight<T> {}

#[derive(Debug, Default)]
pub struct LeftRight<T> {
    left: UnsafeCell<T>,
    right: UnsafeCell<T>,

    leftright: AtomicBool,
    readers_left: AtomicUsize,
    readers_right: AtomicUsize,
}

impl<T: Clone> LeftRight<T> {
    pub fn read<F, R>(&self, f: F) -> R
    where F: Fn(&T) -> R {

        self.arrive();
        let result = if self.is_left_read() {
            unsafe { f(& (*self.left.get())) }
        } else {
            unsafe { f(& (*self.right.get())) }
        };
        self.depart();
        result
    }

    pub fn write<F>(&self, f: F)
    where F: Fn(&mut T) {

        let is_left_read = self.is_left_read();
        if !is_left_read {
            unsafe { f(&mut (*self.left.get()))}
        } else {
            unsafe { f(&mut (*self.right.get()))}
        }

        self.switch();
    }

    fn switch(&self) {
        let is_left_read = self.is_left_read();
        self.leftright.store(!is_left_read, Ordering::SeqCst);

        while self.has_old_readers(!is_left_read) { }

        if is_left_read {
            unsafe { *(self.left.get() as *mut _) = &mut *self.right.get() };
        } else {
            unsafe { *(self.right.get() as *mut _) = &mut *self.left.get() };
        }
    }

    fn arrive(&self) {
        if self.is_left_read() {
            self.readers_left.fetch_add(1, Ordering::SeqCst);
        } else {
            self.readers_right.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn depart(&self) {
        if self.is_left_read() {
            self.readers_left.fetch_sub(1, Ordering::SeqCst);
        } else {
            self.readers_right.fetch_sub(1, Ordering::SeqCst);
        }
    }

    fn has_old_readers(&self, current_reader: bool) -> bool {
        if !current_reader {
            self.readers_left.load(Ordering::SeqCst) > 0
        } else {
            self.readers_right.load(Ordering::SeqCst) > 0
        }
    }

    fn is_left_read(&self) -> bool {
        self.leftright.load(Ordering::SeqCst)
    }
}
