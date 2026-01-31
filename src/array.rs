use std::alloc::{self, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

pub struct Array<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> Array<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), item);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(8))
        } else {
            let new_cap = 2 * self.cap;
            (new_cap, Layout::array::<T>(new_cap))
        };

        let new_layout = new_layout.expect("Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr() as *mut T, self.len) }
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr() as *mut T, self.len) }
    }
}
