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

        let is_left = self.arrive();

        // depending on which side is active, give it the reader
        let result = if is_left {
            unsafe { f(& (*self.left.get())) }
        } else {
            unsafe { f(& (*self.right.get())) }
        };
        self.depart(is_left);
        result
    }

    pub fn write<F>(&self, f: F)
    where F: Fn(&mut T) {

        let is_left_read = self.is_left_read();

        // depending on which side is activ, change the not active side
        if !is_left_read {
            unsafe { f(&mut (*self.left.get()))}
        } else {
            unsafe { f(&mut (*self.right.get()))}
        }

        // switch the reader from left to right or from right to left
        self.leftright.store(!is_left_read, Ordering::SeqCst);

        // wait until all readers switched to the new side
        loop {
            if is_left_read {
                if self.readers_left.load(Ordering::SeqCst) > 0 { continue; } else { break; }
            } else {
                if self.readers_right.load(Ordering::SeqCst) > 0 { continue; } else { break; }
            }
        }

        // finally copy the new data over to the other side
        if is_left_read {
            unsafe { *(self.left.get() as *mut _) = &mut *self.right.get() };
        } else {
            unsafe { *(self.right.get() as *mut _) = &mut *self.left.get() };
        }
    }

    /// Depending on the active side, it increments the counter
    ///
    /// The active side is returned
    /// true -> left
    /// false -> right
    fn arrive(&self) -> bool {
        if self.is_left_read() {
            self.readers_left.fetch_add(1, Ordering::SeqCst);
            true
        } else {
            self.readers_right.fetch_add(1, Ordering::SeqCst);
            false
        }
    }

    /// Decrements the counter for a side depending on the given bool.
    ///
    /// `is_left`
    /// true -> left
    /// false -> right
    fn depart(&self, is_left: bool) {
        if is_left {
            self.readers_left.fetch_sub(1, Ordering::SeqCst);
        } else {
            self.readers_right.fetch_sub(1, Ordering::SeqCst);
        }
    }

    fn is_left_read(&self) -> bool {
        self.leftright.load(Ordering::SeqCst)
    }
}
